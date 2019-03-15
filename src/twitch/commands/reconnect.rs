/// Signals that you should reconnect and rejoin channels after a restart.
///
/// Trovo IRC processes occasionally need to be restarted. When this happens,
/// clients that have requested the IRC v3 trovo.tv/commands capability are
/// issued a RECONNECT. After a short time, the connection is closed. In this
/// case, reconnect and rejoin channels that were on the connection, as you
/// would normally.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Reconnect;
