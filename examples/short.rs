/* in your Cargo.toml
[dependencies]
trovochat = "0.9"                                         # this crate
tokio = { version = "0.2", features = ["full", "macros"] } # you need tokio to run it
*/

// or you can use futures::stream::StreamExt
use tokio::stream::StreamExt as _;

use trovochat::{events, Dispatcher, Runner, Status};

#[tokio::main]
async fn main() {
    let dispatcher = Dispatcher::new();

    let dispatch = dispatcher.clone();
    tokio::task::spawn(async move {
        // subscribe to 'PRIVMSG' events, this is a `Stream`
        let mut privmsgs = dispatch.subscribe::<events::Privmsg>();
        // 'msg' is a trovochat::messages::Privmsg<'static> here
        while let Some(msg) = privmsgs.next().await {
            eprintln!("[{}] {}: {}", msg.channel, msg.name, msg.data);
        }
    });

    let (runner, mut control) = Runner::new(dispatcher.clone(), trovochat::RateLimit::default());

    let (nick, pass) = trovochat::ANONYMOUS_LOGIN;
    let stream = trovochat::connect_easy_tls(&nick, &pass).await.unwrap();

    // run to completion in the background
    let done = tokio::task::spawn(runner.run(stream));

    // 'block' until we're connected
    let ready = dispatcher.wait_for::<events::IrcReady>().await.unwrap();
    eprintln!("your irc name: {}", ready.nickname);

    // the writer is also clonable
    control.writer().join("#museun").await.unwrap();

    // this resolves when the client disconnects
    // or is forced to stop with Control::stop
    // unwrap the JoinHandle
    match done.await.unwrap() {
        // client was disconnected by the server
        Ok(Status::Eof) => {}
        // client was canceled by the user (`stop`)
        Ok(Status::Canceled) => {}
        // the client's connection timed out
        Ok(Status::Timeout) => {}
        // an error was received when trying to read or write
        Err(err) => eprintln!("error!: {}", err),
    };
}
