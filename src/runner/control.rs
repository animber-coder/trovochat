use {super::*, crate::*};

#[derive(Clone)]
/// A control type for writing messages to the client, or stopping it.
pub struct Control {
    pub(super) writer: Writer,
    pub(super) stop: abort::Abort,
}

impl Control {
    /// Get a mutable reference to the [Writer](./encode/struct.AsyncEncoder.html)
    ///
    /// You can clone this to pass around it around
    pub fn writer(&mut self) -> &mut Writer {
        &mut self.writer
    }

    /// Signal the client to stop
    ///
    /// # Example
    /// ```rust
    /// # use trovochat::{Runner, Status, RateLimit, Dispatcher, Connector};
    /// # use tokio::spawn;
    /// # let conn = Connector::new(move || async move { Ok(tokio_test::io::Builder::new().wait(std::time::Duration::from_millis(10000)).build()) });
    /// # let fut = async move {
    /// let (mut runner, control) = Runner::new(Dispatcher::default());
    ///
    /// // calling stop will cause 'run' to return Ok(Status::Canceled)
    /// spawn(async move { control.stop() });
    ///
    /// assert_eq!(runner.run_to_completion(conn).await.unwrap(), Status::Canceled);
    /// # };
    /// # tokio::runtime::Runtime::new().unwrap().block_on(fut);
    /// ```
    pub fn stop(&self) {
        self.stop.cancel()
    }
}

impl std::fmt::Debug for Control {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Control").finish()
    }
}
