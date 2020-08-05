use crate::ng::{FromIrcMessage, InvalidMessage, Validator};
use crate::ng::{IrcMessage, Str, StrIndex};

/// Happens when the IRC connection has been succesfully established
#[derive(Debug, Clone, PartialEq)]
pub struct IrcReady<'t> {
    raw: Str<'t>,
    nickname: StrIndex,
}

impl<'t> IrcReady<'t> {
    raw!();
    str_field!(
        /// The name the server will refer to you as
        nickname
    );
}

impl<'t> FromIrcMessage<'t> for IrcReady<'t> {
    type Error = InvalidMessage;

    fn from_irc(msg: IrcMessage<'t>) -> Result<Self, Self::Error> {
        msg.expect_command(IrcMessage::IRCREADY)?;

        let this = Self {
            nickname: msg.expect_arg_index(0)?,
            raw: msg.raw,
        };

        Ok(this)
    }
}

serde_struct!(IrcReady { raw, nickname });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ng::irc;

    #[test]
    #[cfg(feature = "serde")]
    fn irc_ready_serde() {
        let input = ":tmi.trovo.tv 001 shaken_bot :Welcome, GLHF!\r\n";
        crate::ng::serde::round_trip_json::<IrcReady>(input);
    }

    #[test]
    fn irc_ready() {
        let input = ":tmi.trovo.tv 001 shaken_bot :Welcome, GLHF!\r\n";
        for irc in irc::parse(input).map(|s| s.unwrap()) {
            let msg = IrcReady::from_irc(irc).unwrap();
            assert_eq!(msg.nickname(), "shaken_bot")
        }
    }
}
