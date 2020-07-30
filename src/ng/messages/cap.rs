use super::{FromIrcMessage, InvalidMessage, IrcMessage, Str, StrIndex, Validator};

#[derive(Debug, Clone, PartialEq)]
pub struct Cap<'t> {
    raw: Str<'t>,
    capability: StrIndex,
    acknowledged: bool,
}

impl<'t> Cap<'t> {
    raw!();
    str_field!(capability);

    pub fn acknowledged(&self) -> bool {
        self.acknowledged
    }
}

impl<'a> FromIrcMessage<'a> for Cap<'a> {
    type Error = InvalidMessage;

    fn from_irc(msg: IrcMessage<'a>) -> Result<Self, Self::Error> {
        const ACK: &str = "ACK";

        msg.expect_command(IrcMessage::CAP)?;

        let this = Self {
            capability: msg.expect_data()?,
            acknowledged: msg.expect_arg(1)? == ACK,
            raw: msg.raw,
        };

        Ok(this)
    }
}

impl<'t> serde::Serialize for Cap<'t> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct as _;
        let mut s = serializer.serialize_struct("Cap", 3)?;
        s.serialize_field("raw", &self.raw)?;
        s.serialize_field("capability", &self.raw[self.capability])?;
        s.serialize_field("acknowledged", &self.acknowledged)?;
        s.end()
    }
}

impl<'t, 'de: 't> serde::Deserialize<'de> for Cap<'t> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(crate::ng::RawVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ng::irc;
    use irc::parse;

    #[test]
    fn cap_acknowledged() {
        let input = ":tmi.trovo.tv CAP * ACK :trovo.tv/membership\r\n\
                     :tmi.trovo.tv CAP * ACK :trovo.tv/tags\r\n\
                     :tmi.trovo.tv CAP * ACK :trovo.tv/commands\r\n";
        let expected = &[
            "trovo.tv/membership",
            "trovo.tv/tags",
            "trovo.tv/commands",
        ];
        for (msg, expected) in parse(&input).map(|s| s.unwrap()).zip(expected) {
            let msg = Cap::from_irc(msg).unwrap();
            assert!(msg.acknowledged());
            assert_eq!(msg.capability(), *expected);
        }
    }

    #[test]
    fn cap_failed() {
        let input = ":tmi.trovo.tv CAP * NAK :foobar\r\n";
        for msg in parse(input).map(|s| s.unwrap()) {
            let cap = Cap::from_irc(msg).unwrap();
            assert!(!cap.acknowledged());
            assert_eq!(cap.capability(), "foobar");
        }
    }
}
