/// Capability used to enable extra functionality with the protocol
///
/// Without any of these specified, you will just able to read/write basic messages
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Capability {
    /// Membership capability
    ///
    /// Read basic IRC messages from a Trovo channel allows you to see who is in the channel
    Membership,
    /// Tags capability
    ///
    /// Provides metadata attached to each message
    Tags,
    /// Commands capability
    ///
    /// Enables many Trovo specific commands
    Commands,
    /// ChatRooms capability
    ///
    /// Allows joining and sending/receiving messages in chat rooms
    ChatRooms,
}

impl Capability {
    /// Encode this capability as a string, to be sent to the server
    pub fn encode_as_str(self) -> &'static str {
        match self {
            Capability::Membership => "CAP REQ :trovo.tv/membership",
            Capability::Tags => "CAP REQ :trovo.tv/tags",
            Capability::Commands => "CAP REQ :trovo.tv/commands",
            Capability::ChatRooms => "CAP REQ :trovo.tv/tags trovo.tv/commands",
        }
    }
}
