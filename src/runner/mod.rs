pub(super) use crate::error::Error;

type Rx<T = Vec<u8>> = async_channel::Receiver<T>;

pub mod dispatcher;
#[allow(clippy::module_inception)]
pub mod runner;
pub mod status;
pub mod stream;

pub mod control;

#[doc(hidden)]
pub use crate::events::{Event, EventMapped};

pub mod abort;
