use super::messages::*;
use crate::{EventMap, EventStream, FromIrcMessage, IntoOwned, IrcError, IrcMessage};

use std::convert::Infallible;

#[derive(Debug)]
#[non_exhaustive]
pub enum DispatchError {
    InvalidMessage(IrcError),
    Custom(Box<dyn std::error::Error>),
}

impl DispatchError {
    pub fn custom(err: impl std::error::Error + 'static) -> Self {
        Self::Custom(Box::new(err))
    }
}

impl std::fmt::Display for DispatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidMessage(err) => write!(f, "invalid message: {}", err),
            Self::Custom(err) => write!(f, "unknown error: {}", err),
        }
    }
}

impl std::error::Error for DispatchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidMessage(err) => Some(err),
            Self::Custom(err) => Some(&**err),
        }
    }
}

impl From<IrcError> for DispatchError {
    fn from(msg: IrcError) -> Self {
        Self::InvalidMessage(msg)
    }
}

impl From<Infallible> for DispatchError {
    fn from(_: Infallible) -> Self {
        unreachable!("you cannot produce this error")
    }
}

#[derive(Default)]
pub struct Dispatcher {
    map: EventMap,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe<T: Clone + 'static>(&mut self) -> EventStream<T> {
        self.map.register()
    }

    pub fn dispatch<'a>(&mut self, message: IrcMessage<'a>) -> Result<(), DispatchError> {
        use IrcMessage as M;

        let msg = message.into_owned();
        macro_rules! dispatch {
            ($ty:ty) => {
                self.dispatch_static::<$ty>(msg)?
            };
        }

        match msg.get_command() {
            M::IRC_READY => dispatch!(IrcReady),
            M::READY => dispatch!(Ready),
            M::CAP => dispatch!(Cap),
            M::CLEAR_CHAT => dispatch!(ClearChat),
            M::CLEAR_MSG => dispatch!(ClearMsg),
            M::GLOBAL_USER_STATE => dispatch!(GlobalUserState),
            M::HOST_TARGET => dispatch!(HostTarget),
            M::JOIN => dispatch!(Join),
            M::NOTICE => dispatch!(Notice),
            M::PART => dispatch!(Part),
            M::PING => dispatch!(Ping),
            M::PONG => dispatch!(Pong),
            M::PRIVMSG => dispatch!(Privmsg),
            M::RECONNECT => dispatch!(Reconnect),
            M::ROOM_STATE => dispatch!(RoomState),
            M::USER_NOTICE => dispatch!(UserNotice),
            M::USER_STATE => dispatch!(UserState),
            M::WHISPER => dispatch!(Whisper),
            _ => {
                // TODO user-defined messages

                self.dispatch_static::<IrcMessage>(msg.clone())
                    .expect("identity conversion should be upheld");

                self.dispatch_static::<AllCommands>(msg)
                    .expect("identity conversion should be upheld");
            }
        };

        Ok(())
    }

    pub fn reset(&mut self) {
        self.map.reset()
    }

    fn dispatch_static<T>(&mut self, message: IrcMessage<'static>) -> Result<(), DispatchError>
    where
        T: FromIrcMessage<'static>,
        T: Clone + 'static,
        DispatchError: From<T::Error>,
    {
        self.map.send(T::from_irc(message)?);
        Ok(())
    }
}
