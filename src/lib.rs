#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
/*!
This crate provides a way to interface with [Trovo]'s chat.

Along with the messages as Rust types, it provides methods for sending messages.

# Demonstration
See `examples/demo.rs` for a demo of the api

---
Here's a quick link to the [Event Mapping](./struct.Dispatcher.html#a-table-of-mappings)

[Trovo]: https://www.trovo.tv
*/

#[cfg(all(doctest, feature = "async", feature = "tokio_native_tls"))]
doc_comment::doctest!("../README.md");

static_assertions::assert_cfg!(
    not(all(
        feature = "tokio_native_tls", //
        feature = "tokio_rustls",     //
    )),
    "only a single TLS library can be used."
);

#[macro_use]
#[doc(hidden)]
pub mod macros;

cfg_async! {
    mod runner;
    pub use runner::{
        writer::Writer,
        dispatcher::Dispatcher,
        stream::EventStream,
        runner::Runner,
        control::Control,
        status::Status
    };
}

cfg_async! {
    pub mod events;
}

cfg_async! {
    mod register;
    #[doc(inline)]
    pub use register::register;
}

cfg_async! {
    mod connect;
    #[doc(inline)]
    pub use connect::*;
}

/// Decode messages from a `&str`
pub mod decode;
#[doc(inline)]
pub use decode::{decode, decode_one};

/// Encode data to a `Writer`
pub mod encode;
#[doc(inline)]
pub use encode::Encoder;

/// Common Trovo types
pub mod trovo;

#[doc(inline)]
pub use trovo::*;

pub mod messages;

pub mod sync;

mod parse;
pub use parse::Parse;

mod as_owned;
#[doc(inline)]
pub use as_owned::AsOwned;

mod error;
#[doc(inline)]
pub use error::Error;

/// The Trovo IRC address for non-TLS connections
pub const TROVO_IRC_ADDRESS: &str = "irc.chat.trovo.tv:6667";

/// The Trovo IRC address for TLS connections
pub const TROVO_IRC_ADDRESS_TLS: &str = "irc.chat.trovo.tv:6697";

/// The Trovo WebSocket address for non-TLS connections
pub const TROVO_WS_ADDRESS: &str = "ws://irc-ws.chat.trovo.tv:80";

/// The Trovo WebSocket address for TLS connections
pub const TROVO_WS_ADDRESS_TLS: &str = "wss://irc-ws.chat.trovo.tv:443";

/**
An anonymous login.

You won't be able to send messages, but you can join channels and read messages

# usage
```rust
# use trovochat::{ANONYMOUS_LOGIN, UserConfig};
let (nick, pass) = trovochat::ANONYMOUS_LOGIN;
let _config = UserConfig::builder()
    .name(nick)
    .token(pass)
    .build()
    .unwrap();
```
*/
pub const ANONYMOUS_LOGIN: (&str, &str) = (JUSTINFAN1234, JUSTINFAN1234);
pub(crate) const JUSTINFAN1234: &str = "justinfan1234";

fn simple_user_config(name: &str, token: &str) -> Result<UserConfig, UserConfigError> {
    UserConfig::builder()
        .name(name)
        .token(token)
        .capabilities(&[
            Capability::Membership,
            Capability::Tags,
            Capability::Commands,
        ])
        .build()
}

// TODO see https://github.com/museun/trovochat/issues/91
cfg_async! {
    #[doc(inline)]
    pub mod rate_limit;
    #[doc(inline)]
    pub use rate_limit::RateLimit;
}
