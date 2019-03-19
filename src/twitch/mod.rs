mod badge;
mod color;
mod emotes;

pub use self::badge::{Badge, BadgeKind};
pub use self::color::{trovo_colors as colors, Trovo as TrovoColor, RGB};
pub use self::emotes::Emotes;

/// An assortment of Trovo commands
pub mod commands;

mod capability;
pub use self::capability::Capability;

mod error;
pub use self::error::Error;

mod client;
pub use self::client::Client;

mod writer;
pub use self::writer::Writer;

mod extension;
pub use self::extension::WriterExt;

/// Trovo channel types
mod channel;
pub use self::channel::Channel;

mod mutex;
pub(crate) use self::mutex::mutex_wrapper::MutexWrapper;

#[doc(hidden)]
pub mod userconfig;
pub use self::userconfig::UserConfig;
pub use self::userconfig::UserConfigBuilder;

/// Information gathered during the [`GLOBALUSERSTATE`](./commands/struct.GlobalUserState.html) event
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LocalUser {
    /// Your user id
    pub user_id: u64,
    /// Your display name, if set
    pub display_name: Option<String>,
    /// Your color, if set
    pub color: Option<TrovoColor>,
    /// Your badges
    pub badges: Vec<Badge>,
    /// Your list of emote sets
    pub emote_sets: Vec<u64>,
    /// The capabilities the server acknowledged
    pub caps: Vec<Capability>,
}

/// Messages created by the [`Client`](./struct.Client.html).
///
/// Wraps [`commands`](./commands/index.html)
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Message {
    /// An irc Message
    Irc(crate::irc::types::Message),
    /// Join a channel.
    Join(commands::Join),
    /// Depart from a channel.
    Part(commands::Part),
    /// Send a message to a channel.
    PrivMsg(commands::PrivMsg),
    /// Gain/lose moderator (operator) status in a channel.
    Mode(commands::Mode),
    /// List current chatters in a channel. (begin)
    NamesStart(commands::NamesStart),
    /// List current chatters in a channel. (end)
    NamesEnd(commands::NamesEnd),
    /// Purge a user's typically after a user is banned from chat or timed out.
    ClearChat(commands::ClearChat),
    /// Single message removal on a channel. This is triggered via /delete
    /// <target-msg-id> on IRC.
    ClearMsg(commands::ClearMsg),
    /// Channel starts host mode.
    HostTargetStart(commands::HostTargetStart),
    /// Channel stops host mode.
    HostTargetEnd(commands::HostTargetEnd),
    /// General notices from the server.
    Notice(commands::Notice),
    /// Rejoin channels after a restart.
    Reconnect(commands::Reconnect),
    /// Identifies the channel's chat settings (e.g., slow mode duration).
    RoomState(commands::RoomState),
    /// Announces Trovo-specific events to the channel (e.g., a user's
    /// subscription notification).
    UserNotice(commands::UserNotice),
    /// Identifies a user's chat settings or properties (e.g., chat color)..
    UserState(commands::UserState),
    /// On successful login.
    GlobalUserState(commands::GlobalUserState),
}

pub(crate) mod filter;
pub use filter::Token;
