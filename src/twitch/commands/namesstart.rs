/// List current chatters in a channel. (marks the start begin)
///
/// If there are more than 1000 chatters in a room, NAMES return only the list
/// of operator privileges currently in the room.
///
/// The server will send these until it sends a NamesEnd for the same channel
///
/// Listen for this and keep track of the users and once you received NamedEnd
/// you've gotten all of the users
#[derive(Debug, PartialEq, Clone)]
pub struct NamesStart {
    /// Your user for this event
    pub user: String,
    /// The channel this event is happening on
    pub channel: String,
    /// List of users returned by this
    pub users: Vec<String>,
}
