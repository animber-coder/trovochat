// note this uses `smol`. you can use `tokio` or `async_std` or `async_io` if you prefer.
// this is a helper module to reduce code deduplication
// extensions to the Privmsg type
use trovochat::PrivmsgExt as _;
use trovochat::{
    messages::{Commands, Privmsg},
    runner::{AsyncRunner, NotifyHandle, Status},
    UserConfig,
};

// this is a helper module to reduce code deduplication
mod include;
use crate::include::{channels_to_join, get_user_config};

use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    // you'll need a user configuration
    let user_config = get_user_config()?;
    // and some channels to join
    let channels = channels_to_join()?;

    let start = std::time::Instant::now();

    let mut bot = Bot::default()
        .with_command("!hello", |args: Args| {
            let output = format!("hello {}!", args.msg.name());
            // We can 'reply' to this message using a writer + our output message
            args.writer.reply(args.msg, &output).unwrap();
        })
        .with_command("!uptime", move |args: Args| {
            let output = format!("its been running for {:.2?}", start.elapsed());
            // We can send a message back (without quoting the sender) using a writer + our output message
            args.writer.say(args.msg, &output).unwrap();
        })
        .with_command("!quit", move |args: Args| {
            // because we're using sync stuff, turn async into sync with smol!
            smol::block_on(async move {
                // calling this will cause read_message() to eventually return Status::Quit
                args.quit.notify().await
            });
        });

    // run the bot in the executor
    smol::block_on(async move { bot.run(&user_config, &channels).await })
}

struct Args<'a, 'b: 'a> {
    msg: &'a Privmsg<'b>,
    writer: &'a mut trovochat::Writer,
    quit: NotifyHandle,
}

trait Command: Send + Sync {
    fn handle(&mut self, args: Args<'_, '_>);
}

impl<F> Command for F
where
    F: Fn(Args<'_, '_>),
    F: Send + Sync,
{
    fn handle(&mut self, args: Args<'_, '_>) {
        (self)(args)
    }
}

#[derive(Default)]
struct Bot {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Bot {
    // add this command to the bot
    fn with_command(mut self, name: impl Into<String>, cmd: impl Command + 'static) -> Self {
        self.commands.insert(name.into(), Box::new(cmd));
        self
    }

    // run the bot until its done
    async fn run(&mut self, user_config: &UserConfig, channels: &[String]) -> anyhow::Result<()> {
        // this can fail if DNS resolution cannot happen
        let connector = trovochat::connector::smol::Connector::trovo()?;

        let mut runner = AsyncRunner::connect(connector, user_config).await?;
        println!("connecting, we are: {}", runner.identity.username());

        for channel in channels {
            println!("joining: {}", channel);
            if let Err(err) = runner.join(channel).await {
                eprintln!("error while joining '{}': {}", channel, err);
            }
        }

        // if you store this somewhere, you can quit the bot gracefully
        // let quit = runner.quit_handle();

        println!("starting main loop");
        self.main_loop(&mut runner).await
    }

    // the main loop of the bot
    async fn main_loop(&mut self, runner: &mut AsyncRunner) -> anyhow::Result<()> {
        // this is clonable, but we can just share it via &mut
        // this is rate-limited writer
        let mut writer = runner.writer();
        // this is clonable, but using it consumes it.
        // this is used to 'quit' the main loop
        let quit = runner.quit_handle();

        loop {
            // this drives the internal state of the crate
            match runner.next_message().await? {
                // if we get a Privmsg (you'll get an Commands enum for all messages received)
                Status::Message(Commands::Privmsg(pm)) => {
                    // see if its a command and do stuff with it
                    if let Some(cmd) = Self::parse_command(pm.data()) {
                        if let Some(command) = self.commands.get_mut(cmd) {
                            println!("dispatching to: {}", cmd.escape_debug());

                            let args = Args {
                                msg: &pm,
                                writer: &mut writer,
                                quit: quit.clone(),
                            };

                            command.handle(args);
                        }
                    }
                }
                // stop if we're stopping
                Status::Quit | Status::Eof => break,
                // ignore the rest
                Status::Message(..) => continue,
            }
        }

        println!("end of main loop");
        Ok(())
    }

    fn parse_command(input: &str) -> Option<&str> {
        if !input.starts_with('!') {
            return None;
        }
        input.splitn(2, ' ').next()
    }
}
