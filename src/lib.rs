#![allow(
    clippy::missing_const_for_fn,
    clippy::redundant_pub_crate,
    clippy::use_self
)]
#![deny(
    deprecated_in_future,
    exported_private_dependencies,
    future_incompatible,
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_debug_implementations,
    missing_docs,
    private_in_public,
    rust_2018_compatibility,
    // rust_2018_idioms, // this complains about elided lifetimes.
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(docsrs, feature(doc_cfg), feature(doc_alias))]
/*!

This crate provides a way to interface with [Trovo](https://dev.trovo.tv/docs/irc)'s chat (via IRC).

Along with the messages as Rust types, it provides methods for sending messages.

---

By default, this crate depends on zero external crates -- but it makes it rather limited in scope.

This allows parsing, and decoding/encoding to standard trait types (`std::io::{Read, Write}`).

The use the `AsyncRunner` (an async-event loop) and related helpers, you must able the `async` feature.

***NOTE*** This is a breaking change from `0.12` which had the async stuff enabled by default.

```toml
trovochat = { version = "0.13", features = ["async"] }
```
To use a specific `TcpStream`/`TlStream` refer to the runtime table below.

---

For trovo types:
* [trovo]
* [messages]
* [commands]
---
For the 'irc' types underneath it all:
* [irc]
---
For an event loop:
* [runner]
---
For just decoding messages:
* [decoder]
---
For just encoding messages:
* [encoder]
---

[runner]: runner/index.html
[encoder]: encoder/index.html
[decoder]: decoder/index.html
[trovo]: trovo/index.html
[messages]: messages/index.html
[commands]: commands/index.html
[irc]: irc/index.html
*/

macro_rules! cfg_async {
    ($($item:item)*) => {
        $(
            #[cfg(any(feature = "async"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
            $item
        )*
    };
}

/// The Trovo IRC address for non-TLS connections
pub const TROVO_IRC_ADDRESS: &str = "irc.chat.trovo.tv:6667";

/// The Trovo IRC address for TLS connections
pub const TROVO_IRC_ADDRESS_TLS: &str = "irc.chat.trovo.tv:6697";

/// The Trovo WebSocket address for non-TLS connections
pub const TROVO_WS_ADDRESS: &str = "ws://irc-ws.chat.trovo.tv:80";

/// The Trovo WebSocket address for TLS connections
pub const TROVO_WS_ADDRESS_TLS: &str = "wss://irc-ws.chat.trovo.tv:443";

/// A TLS domain for Trovo
pub const TROVO_TLS_DOMAIN: &str = "irc.chat.trovo.tv";

/// An anonymous login.
pub const ANONYMOUS_LOGIN: (&str, &str) = (JUSTINFAN1234, JUSTINFAN1234);
pub(crate) const JUSTINFAN1234: &str = "justinfan1234";

#[macro_use]
#[allow(unused_macros)]
mod macros;

pub mod decoder;
pub use decoder::{DecodeError, Decoder};
cfg_async! { pub use decoder::AsyncDecoder; }

pub mod encoder;
pub use encoder::Encoder;
cfg_async! { pub use encoder::AsyncEncoder; }

cfg_async! {
    /// A boxed `Future` that is `Send + Sync`
    pub type BoxedFuture<T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + Sync>>;

    /// An AsyncWriter over an MpscWriter
    pub type Writer = crate::writer::AsyncWriter<crate::writer::MpscWriter>;
}

cfg_async! { pub mod connector; }
cfg_async! { pub mod writer; }
cfg_async! { pub mod channel; }

pub mod runner;
pub use runner::{Error as RunnerError, Status};
cfg_async! { pub use runner::AsyncRunner; }

pub mod rate_limit;

pub mod commands;
pub mod messages;

pub mod irc;
pub use irc::{IrcMessage, MessageError};

#[doc(inline)]
pub use irc::{FromIrcMessage, IntoIrcMessage};

pub mod trovo;
pub use trovo::UserConfig;

mod encodable;
pub use encodable::Encodable;

pub mod maybe_owned;
pub use maybe_owned::IntoOwned;
use maybe_owned::{MaybeOwned, MaybeOwnedIndex};

mod validator;
pub use validator::Validator;

mod ext;
#[cfg(feature = "serde")]
mod serde;
mod util;

pub use ext::PrivmsgExt;

// /// Prelude with common types
// pub mod prelude {
//     pub use crate::irc::{IrcMessage, TagIndices, Tags};
//     pub use crate::rate_limit::RateClass;
//     pub use crate::Encodable;
//     pub use crate::{commands, messages, trovo};
//     pub use crate::{Decoder, Encoder};

//     cfg_async! {
//         pub use crate::decoder::AsyncDecoder;
//         pub use crate::encoder::AsyncEncoder;
//         pub use crate::runner::{AsyncRunner, Identity, NotifyHandle, Status};
//     }
// }
