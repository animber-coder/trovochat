// NOTE: this demo requires `--features="tokio/full tokio-util"`.
use trovochat::{
    commands, connector, messages,
    runner::{AsyncRunner, Status},
    UserConfig,
};

// this is a helper module to reduce code deduplication
mod include;
use crate::include::{channels_to_join, get_user_config, main_loop};

async fn connect(user_config: &UserConfig, channels: &[String]) -> anyhow::Result<AsyncRunner> {
    // create a connector using ``tokio``, this connects to Trovo.
    // you can provide a different address with `custom`
    let connector = connector::tokio::Connector::trovo();

    println!("we're connecting!");
    // create a new runner. this is a provided async 'main loop'
    // this method will block until you're ready
    let mut runner = AsyncRunner::connect(connector, user_config).await?;
    println!("..and we're connected");

    // and the identity Trovo gave you
    println!("our identity: {:#?}", runner.identity);

    for channel in channels {
        // the runner itself has 'blocking' join/part to ensure you join/leave a channel.
        // these two methods return whether the connection was closed early.
        // we'll ignore it for this demo
        println!("attempting to join '{}'", channel);
        let _ = runner.join(&channel).await?;
        println!("joined '{}'!", channel);
    }

    Ok(runner)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // create a user configuration
    let user_config = get_user_config()?;
    // get some channels to join from the environment
    let channels = channels_to_join()?;

    // connect and join the provided channels
    let runner = connect(&user_config, &channels).await?;

    // you can get a handle to shutdown the runner
    let quit_handle = runner.quit_handle();

    // you can get a clonable writer
    let mut writer = runner.writer();

    // spawn something off in the background that'll exit in 10 seconds
    tokio::spawn({
        let mut writer = writer.clone();
        let channels = channels.clone();
        async move {
            println!("in 10 seconds we'll exit");
            tokio::time::delay_for(std::time::Duration::from_secs(10)).await;

            // send one final message to all channels
            for channel in channels {
                let cmd = commands::privmsg(&channel, "goodbye, world");
                writer.encode(cmd).await.unwrap();
            }

            println!("sending quit signal");
            quit_handle.notify().await;
        }
    });

    // you can encode all sorts of 'commands'
    for channel in &channels {
        writer
            .encode(commands::privmsg(channel, "hello world!"))
            .await?;
    }

    println!("starting main loop");
    // your 'main loop'. you'll just call next_message() until you're done
    main_loop(runner).await
}
