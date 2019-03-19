/// An error that the [`Client`](./struct.Client.html) can return
#[derive(Debug)]
pub enum Error {
    /// Connection could not be established
    Connect(std::io::Error),
    /// Could not register with the provided [UserConfig](./struct.UserConfig.html)
    Register(Box<Self>),
    /// Could not write
    Write(std::io::Error),
    /// Could not read
    Read(std::io::Error),
    /// Invalid message received from Trovo
    InvalidMessage(String),
    /// Invalid Nick/Pass combination
    InvalidRegistration,
    /// Channel name provided was empty
    EmptyChannelName,
    /// Cannot read. This probably means you need to reconnect.
    CannotRead,
    /// Tags are required for this functionality
    TagsRequired,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Connect(err) => write!(f, "cannot connect: {}", err),
            Error::Register(err) => write!(f, "cannot send initial registration: {}", err),
            Error::Write(err) => write!(f, "cannot write: {}", err),
            Error::Read(err) => write!(f, "cannot read: {}", err),
            Error::InvalidMessage(raw) => {
                write!(f, "invalid message, from '{}' (trimmed)", raw.trim())
            }
            Error::InvalidRegistration => {
                write!(f, "invalid registration. check the `token` and `nick`")
            }
            Error::EmptyChannelName => write!(f, "empty channel name provided"),
            Error::CannotRead => write!(f, "cannot read, client should quit now"),
            Error::TagsRequired => write!(f, "tags are required to do that"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Connect(err) | Error::Write(err) | Error::Read(err) => {
                Some(err as &(dyn std::error::Error))
            }
            Error::Register(err) => Some(err as &(dyn std::error::Error)),
            _ => None,
        }
    }
}
