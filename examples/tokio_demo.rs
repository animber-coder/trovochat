// NOTE: this demo requires `--features="tokio/full tokio-util"`.
use anyhow::Context;

use trovochat::{
    commands, connector, messages,
    runner::{AsyncRunner, Status},
    UserConfig,
};

fn get_env_var(key: &str) -> anyhow::Result<String> {
    std::env::var(key).with_context(|| format!("please set `{}`", key))
}

fn get_user_config() -> anyhow::Result<trovochat::UserConfig> {
    let name = get_env_var("TROVO_NAME")?;
    let token = get_env_var("TROVO_TOKEN")?;

    // you need a `UserConfig` to connect to Trovo
    let config = UserConfig::builder()
        // the name of the associated trovo account
        .name(name)
        // and the provided OAuth token
        .token(token)
        // and enable all of the advanced message signaling from Trovo
        .enable_all_capabilities()
        .build()?;

    Ok(config)
}

fn channels_to_join() -> anyhow::Result<Vec<String>> {
    let channels = get_env_var("TROVO_CHANNEL")?
        .split(',')
        .map(ToString::to_string)
        .collect();
    Ok(channels)
}

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

async fn handle_message(msg: messages::AllCommands<'_>) {
    use messages::AllCommands::*;
    // All sorts of messages
    match msg {
        // This is the one users send to channels
        Privmsg(msg) => println!("[{}] {}: {}", msg.channel(), msg.name(), msg.data()),

        // This one is special, if trovo adds any new message
        // types, this will catch it until future releases of
        // this crate add them.
        Raw(_) => {}

        // these three you'll normally never see. 'connect' uses
        // them internally.
        IrcReady(_) => {}
        Ready(_) => {}
        Cap(_) => {}

        // and a bunch of other messages you may be interested in
        ClearChat(_) => {}
        ClearMsg(_) => {}
        GlobalUserState(_) => {}
        HostTarget(_) => {}
        Join(_) => {}
        Notice(_) => {}
        Part(_) => {}
        Ping(_) => {}
        Pong(_) => {}
        Reconnect(_) => {}
        RoomState(_) => {}
        UserNotice(_) => {}
        UserState(_) => {}
        Whisper(_) => {}

        _ => {}
    }
}

async fn main_loop(mut runner: AsyncRunner) -> anyhow::Result<()> {
    loop {
        match runner.next_message().await? {
            // this is the parsed message -- across all channels (and notifications from Trovo)
            Status::Message(msg) => {
                handle_message(msg).await;
            }

            // you signaled a quit
            Status::Quit => {
                println!("we signaled we wanted to quit");
                break;
            }
            // the connection closed normally
            Status::Eof => {
                println!("we got a 'normal' eof");
                break;
            }
        }
    }

    Ok(())
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
    writer
        .encode(commands::privmsg("#museun", "hello world!"))
        .await?;

    println!("starting main loop");
    // your 'main loop'. you'll just call next_message() until you're done
    main_loop(runner).await
}
