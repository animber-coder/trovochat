use {super::*, crate::*};

use std::sync::Arc;
use tokio::sync::Mutex;

use tokio::prelude::*;

pub struct Runner {
    dispatcher: Dispatcher,
    receiver: Rx,
    writer: Writer,
    abort: abort::Abort,
}

impl Runner {
    /**
    Create a new client runner with this [`Dispatcher`][dispatcher]

    # Returns
    The [`Runner`]() and a [`Control`][control] type

    [control]: ./struct.Control.html
    [dispatcher]: ./struct.Dispatcher.html
    */
    pub fn new(dispatcher: Dispatcher, rate_limit: RateLimit) -> (Self, Control) {
        let (sender, receiver) = mpsc::channel(64);
        let abort = abort::Abort::default();

        let writer = Writer::new(writer::MpscWriter::new(sender))
            .with_rate_limiter(Arc::new(Mutex::new(rate_limit)));

        let control = Control {
            writer: writer.clone(),
            stop: abort.clone(),
        };

        let this = Self {
            receiver,
            dispatcher,
            writer,
            abort,
        };

        (this, control)
    }

    /**
    Run to completion, dispatching messages to the subscribers.

    This returns a future. You should await this future at the end of your code
    to keep the runtime active until the client closes.

    # Interacting with the runner
    You can interact with the runner via the `Control` type returned by [`Runner::new`](#method.new).

    To _stop_ this early, you can use the [`Control::stop`][stop] method.

    To get a _writer_, you can use the [`Control::writer`][writer] method.

    # Returns after resolving the future
    * An [error][error] if one was encountered while in operation
    * [`Ok(Status::Eof)`][eof] if it ran to completion
    * [`Ok(Status::Canceled)`][cancel] if the associated [`Control::stop`][stop] was called

    [error]: ./enum.Error.html
    [eof]: ./enum.Status.html#variant.Eof
    [cancel]: ./enum.Status.html#variant.Canceled
    [stop]: ./struct.Control.html#method.stop
    [writer]: ./struct.Control.html#method.writer
    */
    pub async fn run<IO>(mut self, io: IO) -> Result<Status, Error>
    where
        IO: AsyncRead + AsyncWrite + Send + Sync + Unpin + 'static,
    {
        use futures::prelude::*;
        let mut stream = tokio::io::BufStream::new(io);
        let mut buffer = String::with_capacity(1024);

        let mut ping = self
            .dispatcher
            .subscribe_internal::<crate::events::Ping>(true);

        let mut out = self.writer;

        loop {
            tokio::select! {
                // Abort notification
                _ = self.abort.wait_for() => {
                    let _ = self.dispatcher.clear_subscriptions_all();
                    break Ok(Status::Canceled)
                }

                // Auto-ping
                Some(msg) = ping.next() => {
                    if out.pong(&msg.token).await.is_err() {
                        break Ok(Status::Eof);
                    }
                }

                // Read half
                Ok(n) = &mut stream.read_line(&mut buffer) => {
                    if n == 0 {
                        break Ok(Status::Eof)
                    }

                    for msg in decode(&buffer) {
                        let msg = msg?;
                        log::trace!("< {}", msg.raw.escape_debug());
                        self.dispatcher.dispatch(&msg);
                    }
                    buffer.clear();
                },

                // Write half
                Some(data) = &mut self.receiver.next() => {
                    log::trace!("> {}", std::str::from_utf8(&data).unwrap().escape_debug());
                    stream.write_all(&data).await?;
                    stream.flush().await?
                },

                // All of the futures are dead, so the loop should end
                else => { break Ok(Status::Eof) }
            }
        }
    }
}

impl std::fmt::Debug for Runner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Runner").finish()
    }
}
