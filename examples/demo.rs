/* in your Cargo.toml
[dependencies]
trovochat = "0.8"                               # this crate
tokio = { version = "0.2", features = ["full"] } # you need tokio to run it
*/

use tokio::stream::StreamExt as _;

#[tokio::main]
async fn main() {
    let (nick, pass) = (
        // trovo name
        std::env::var("TROVO_NICK").unwrap(),
        // oauth token for trovo name
        std::env::var("TROVO_PASS").unwrap(),
    );

    // putting this in the env so people don't join my channel when running this
    let channel = std::env::var("TROVO_CHANNEL").unwrap();

    // connect via tcp with tls with this nick and password
    let stream = trovochat::connect_easy_tls(&nick, &pass).await.unwrap();

    // split the stream
    let (read, write) = tokio::io::split(stream);

    // make a client. the client is clonable
    let client = trovochat::Client::new();

    // get a future that resolves when the client is done reading, fails to read/write or is stopped
    let done = client.run(read, write);

    // subscribe to some an event streams:

    // for privmsg (what users send to channels)
    // this event dispatcher is behind a RAII guard, so make sure you drop it.
    // otherwise it'll block the client until its dropped
    let mut privmsg = client
        .dispatcher()
        .await
        .subscribe::<trovochat::events::Privmsg>();

    // spawn a task to consume the stream
    tokio::task::spawn(async move {
        while let Some(msg) = privmsg.next().await {
            eprintln!("[{}] {}: {}", msg.channel, msg.name, msg.data);
        }
    });

    // for join (when a user joins a channel)
    let mut join = client
        .dispatcher()
        .await
        .subscribe::<trovochat::events::Join>();

    tokio::task::spawn(async move {
        while let Some(msg) = join.next().await {
            // we've joined a channel
            if msg.name == nick {
                eprintln!("you joined {}", msg.channel);
                break; // returning/dropping the stream un-subscribes it
            }
        }
    });

    // for privmsg again
    let mut bot = client
        .dispatcher()
        .await
        .subscribe::<trovochat::events::Privmsg>();

    // we can move the client to another task by cloning it
    let bot_client = client.clone();
    tokio::task::spawn(async move {
        let mut writer = bot_client.writer();
        while let Some(msg) = bot.next().await {
            match msg.data.split(" ").next() {
                Some("!quit") => {
                    // causes the client to shutdown
                    bot_client.stop().await.unwrap();
                }
                Some("!hello") => {
                    let response = format!("hello {}!", msg.name);
                    // send a message in response
                    if let Err(_err) = writer.privmsg(&msg.channel, &response).await {
                        // we ran into a write error, we should probably leave this task
                        return;
                    }
                }
                _ => {}
            }
        }
    });

    // wait for the trovo connection to be 'registered'
    let _ready = client.wait_for_irc_ready().await.unwrap();
    // after this point its safe to join / send messages

    // get a clonable writer from the client
    // join a channel, methods on writer return false if the client is disconnected
    if let Err(err) = client.writer().join(&channel).await {
        match err {
            trovochat::Error::InvalidChannel(..) => {
                eprintln!("you cannot join a channel with an empty name. demo is ending");
                std::process::exit(1);
            }
            _ => {
                // we'll get an error if we try to write to a disconnected client.
                // if this happens, you should shutdown your tasks
            }
        }
    }

    // you can clear subscriptions with
    // client.dispatcher().await.clear_subscriptions::<event::Join>()
    // or all subscriptions
    // client.dispatcher().await.clear_subscriptions_all()

    // you can get the number of active subscriptions with
    // client.dispatcher().await.count_subscribers::<event::Join>()
    // or all subscriptions
    // client.dispatcher().await.count_subscribers_all()

    // await for the client to be done
    match done.await {
        Ok(trovochat::client::Status::Eof) => {
            eprintln!("done!");
        }
        Ok(trovochat::client::Status::Canceled) => {
            eprintln!("client was stopped by user");
        }
        Err(err) => {
            eprintln!("error: {}", err);
        }
    }

    // note you should wait for all of your tasks to join before exiting
    // but we detached them to make this shorter

    // another way would be to clear all subscriptions
    // clearing the subscriptions would close each event stream
    client.dispatcher().await.clear_subscriptions_all();
}
