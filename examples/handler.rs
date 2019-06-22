use std::net::TcpStream;
use std::sync::Arc;

use trovochat::irc::types::Message as IrcMessage;
use trovochat::{commands::*, Client, Handler, Message, UserConfig, Writer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // connect to trovo via a tcp stream, creating a read/write pair
    let (read, write) = {
        let stream = TcpStream::connect(trovochat::TROVO_IRC_ADDRESS)?;
        (stream.try_clone()?, stream)
    };

    // create synchronous 'adapters' for the tcpstream
    let (read, write) = trovochat::sync_adapters(read, write);

    // create a config
    let conf = user_config();

    // create a client from the read/write pair
    let mut client = Client::new(read, write);

    // register with the server, using the config
    client.register(conf)?;

    // wait until the server tells us who we are
    let local = client.wait_for_ready()?;
    let mention = format!("@{}", local.display_name.unwrap());

    // log Message and IrcMessages
    struct Foo;
    impl Handler for Foo {
        fn on_message(&mut self, msg: Arc<Message>) {
            // this also contains `IrcMessage`
            eprintln!("parsed message: {:?}", msg)
        }
        fn on_irc_message(&mut self, msg: Arc<IrcMessage>) {
            eprintln!("irc message: {:?}", msg)
        }
    }

    // reply to mentions, log if someone joins "#museun"
    struct Bar {
        mention: String,
        writer: Writer,
    }

    impl Handler for Bar {
        fn on_priv_msg(&mut self, msg: Arc<PrivMsg>) {
            if msg.message().contains(&self.mention) {
                self.writer.send(msg.channel(), "VoHiYo").unwrap();
            }
        }
        fn on_join(&mut self, msg: Arc<Join>) {
            if msg.channel() == "#museun" {
                eprintln!("{:?} joined.", msg.user());
            }
        }
    }

    // add the Foo handler
    let _ = client.handler(Foo {});

    // add the Bar handler
    let writer = client.writer();
    let _ = client.handler(Bar { writer, mention });

    // get the writer, this is threadsafe and writers to the same internal buffer
    let w = client.writer();

    // join a channel
    w.join("museun")?;

    // send a message to the channel
    w.send("museun", "HeyGuys")?;

    // run until an error
    client.run()?;

    // (would have to box the error up after turning it into a trait object
    //  so lets just ? then Ok)
    Ok(())
}

fn user_config() -> UserConfig {
    let (nick, pass) = (var("MY_TROVO_NICK"), var("MY_TROVO_PASS"));
    let config = UserConfig::builder()
        .nick(nick)
        .token(pass)
        .membership()
        .commands()
        .tags();
    config.build().unwrap()
}

fn var(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("please set the env var `{}`", key))
}
