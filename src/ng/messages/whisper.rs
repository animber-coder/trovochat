use super::{AsOwned, FromIrcMessage, InvalidMessage, IrcMessage, Reborrow, Str, Validator};

#[derive(Debug, Clone)]
pub struct Whisper<'a> {
    s: &'a (),
}
