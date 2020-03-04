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

[Trovo]: https://www.trovo.tv
*/

#[macro_use]
#[doc(hidden)]
pub mod macros;

cfg_async! {
    pub mod client;
    pub use client::Client;
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

cfg_async! {
    pub mod events;
}

pub mod messages;

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

pub mod sync;

cfg_async! {
    #[doc(inline)]
    pub mod rate_limit;
}

/// A trait for parsing messages
///
/// # Example
/// ```rust
/// # use trovochat::*;
/// # use trovochat::messages::*;
/// # use std::borrow::Cow;
///
/// let input = ":test!test@test JOIN #museun\r\n";
/// let message: Raw<'_> = decode::decode(&input).next().unwrap().unwrap();
/// let join: Join<'_> = Join::parse(&message).unwrap();
/// assert_eq!(join, Join { channel: Cow::Borrowed("#museun"), name: Cow::Borrowed("test") });
/// ```
pub trait Parse<T>: Sized + private::ParseSealed<T> {
    /// Tries to parse the input as this message
    fn parse(input: T) -> Result<Self, crate::messages::InvalidMessage>;
}

mod as_owned;
#[doc(inline)]
pub use as_owned::AsOwned;

mod private {
    pub trait ParseSealed<E> {}
    impl<T: crate::Parse<E>, E: Sized> ParseSealed<E> for T {}
}

mod error;
#[doc(inline)]
pub use error::Error;

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
