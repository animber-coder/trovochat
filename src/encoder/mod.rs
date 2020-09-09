//! # Encoding utilies
//!
//! ## Using the [`Encodable`][encodable] trait
//! Many [`commands`][commands] are provided that can be encoded to a writer.
//!
//! ```
//! use trovochat::{Encodable, commands};
//! // or anything that impls `std::io::Write`
//! let mut buf = vec![];
//! // the commands produce 0-copy types
//! let join_cmd = commands::join("museun");
//!
//! // which implement encode, which lets them to be written to a std:::Write
//! join_cmd.encode(&mut buf).unwrap();
//!
//! // join, for example, makes sure '#' is included in the channel name.
//! let string = std::str::from_utf8(&buf).unwrap();
//! assert_eq!(string, "JOIN #museun\r\n");
//! ```
//!
//! ## Using an Encoder
//! This crate provides composable types (Writers/Encoders) which can be used with the [`Encodable`][encodable] trait.
//! The types come in both `Sync` and `Async` styles.
//!
//! ```
//! use trovochat::commands;
//!
//! let mut buf = vec![];
//! let mut enc = trovochat::Encoder::new(&mut buf);
//! enc.encode(commands::join("museun")).unwrap();
//!
//! use std::io::Write as _;
//! enc.write_all(b"its also a writer\r\n").unwrap();
//! enc.flush().unwrap();
//!
//! let string = std::str::from_utf8(&buf).unwrap();
//! assert_eq!(string, "JOIN #museun\r\nits also a writer\r\n");
//! ```
//! [encodable]: ../trait.Encodable.html
//! [commands]: ../commands/index.html

cfg_async! {
    mod r#async;
    pub use r#async::*;
}

mod sync;
pub use sync::*;
