//! Decoding utilities.
//!
//! A decoder lets you decode messages from an `std::io::Read` (or `futures::io::AsyncRead` for async) in either an iterative fashion, or one-by-one.
//!
//! When not using the Iterator (or Stream), you'll get a borrowed message from the reader that is valid until the next read.
//!
//! With the Iterator (or Stream) interface, it'll return an owned messages.
//!
//! This crate provides both 'Sync' (Iterator based) and 'Async' (Stream based) decoding.
//! * sync: [`Decoder`][decoder]
//! * async: [`AsyncDecoder`][async_decoder]
//!
//! # Borrowed messages
//! ```
//! let input = "@key1=val;key2=true :user!user@user PRIVMSG #some_channel :\x01ACTION hello world\x01\r\n";
//! let mut reader = std::io::Cursor::new(input.as_bytes());
//!
//! // you can either &mut borrow the reader, or let the Decoder take ownership.
//! // ff it takes ownership you can retrieve the inner reader later.
//! let mut decoder = trovochat::Decoder::new(&mut reader);
//!
//! // returns whether the message was valid
//! // this'll block until it can read a 'full' message (e.g. one delimited by `\r\n`).
//! let msg = decoder.read_message().unwrap();
//!
//! // msg is borrowed until the next `read_message()`
//! // you can turn a borrowed message into an owned message by using the trovochat::IntoOwned trait.
//! use trovochat::IntoOwned as _;
//! let owned: trovochat::IrcMessage<'static> = msg.into_owned();
//! ```
//!
//! # Owned messages
//! ```
//! let input = "@key1=val;key2=true :user!user@user PRIVMSG #some_channel :\x01ACTION hello world\x01\r\n";
//! let mut reader = std::io::Cursor::new(input.as_bytes());
//!
//! // you can either &mut borrow the reader, or let the Decoder take ownership.
//! // ff it takes ownership you can retrieve the inner reader later.
//! for msg in trovochat::Decoder::new(&mut reader) {
//!     // this yields whether the message was valid or not
//!     // this'll block until it can read a 'full' message (e.g. one delimited by `\r\n`).
//!
//!     // notice its already owned here (denoted by the 'static lifetime)
//!     let msg: trovochat::IrcMessage<'static> = msg.unwrap();
//! }
//! ```
//! [decoder]: struct.Decoder.html
//! [async_decoder]: struct.AsyncDecoder.html
//!

cfg_async! {
    mod r#async;
    pub use r#async::*;
}

mod sync;
pub use sync::*;
