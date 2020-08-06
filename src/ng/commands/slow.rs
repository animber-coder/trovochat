use crate::ng::Encodable;
use std::io::{Result, Write};

use super::ByteWriter;

/// Enables slow mode (limit how often users may send messages).
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
pub struct Slow<'a> {
    pub(crate) channel: &'a str,
    pub(crate) duration: usize,
}

/// Enables slow mode (limit how often users may send messages).
///
/// Duration (optional, **default=120**) must be a positive number of seconds.
///
/// Use [slow_off] to disable.
///
/// [slow_off]: ./struct.Encoder.html#method.slow_off
pub fn slow(channel: &str, duration: impl Into<Option<usize>>) -> Slow<'_> {
    Slow {
        channel,
        duration: duration.into().unwrap_or(120),
    }
}

impl<'a> Encodable for Slow<'a> {
    fn encode<W: Write + ?Sized>(&self, buf: &mut W) -> Result<()> {
        ByteWriter::new(buf).command(self.channel, &[&"/slow", &self.duration.to_string()])
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn slow_encode() {
        test_encode(slow("#museun", Some(42)), "PRIVMSG #museun :/slow 42\r\n");
        test_encode(slow("#museun", 42), "PRIVMSG #museun :/slow 42\r\n");
        test_encode(slow("#museun", None), "PRIVMSG #museun :/slow 120\r\n");
    }

    #[test]
    #[cfg(feature = "serde")]
    fn slow_serde() {
        test_serde(slow("#museun", Some(42)), "PRIVMSG #museun :/slow 42\r\n");
        test_serde(slow("#museun", 42), "PRIVMSG #museun :/slow 42\r\n");
        test_serde(slow("#museun", None), "PRIVMSG #museun :/slow 120\r\n");
    }
}
