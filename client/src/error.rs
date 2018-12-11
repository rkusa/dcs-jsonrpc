use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Recv(std::sync::mpsc::RecvError),
    Send(std::sync::mpsc::SendError<std::vec::Vec<u8>>),
    Json(serde_json::Error),
    Rpc(dcsjsonrpc_common::RpcError),
    GroupGone(String),
    UnitGone(String),
    StaticGone(String),
    AddGroupTimeout,
    AddStaticTimeout,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Error::*;
        use std::error::Error;

        match self {
            GroupGone(ref id) | UnitGone(ref id) | StaticGone(ref id) => write!(f, "{} does not exist anymore", id)?,
            _ => write!(f, "Error: {}", self.description())?,
        }

        let mut cause: Option<&dyn error::Error> = self.cause();
        while let Some(err) = cause {
            write!(f, "  -> {}", err)?;
            cause = err.cause();
        }

        Ok(())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;

        match *self {
            Io(_) => "Error in TCP connection",
            Recv(_) => "Error receiving from channel",
            Send(_) => "Error sending to channel",
            Json(_) => "Error serializing or deserializing JSON",
            Rpc(ref err) => err.description(),
            GroupGone(_) => "Group does not exist anymore",
            UnitGone(_) => "Unit does not exist anymore",
            StaticGone(_) => "Static does not exist anymore",
            AddGroupTimeout => {
                "A newly added group did not exist 1 second after its supposed spawn"
            },
            AddStaticTimeout => {
                "A newly added statics did not exist 1 second after its supposed spawn"
            }
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        use self::Error::*;

        match *self {
            Io(ref err) => Some(err),
            Recv(ref err) => Some(err),
            Send(ref err) => Some(err),
            Json(ref err) => Some(err),
            Rpc(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<std::sync::mpsc::RecvError> for Error {
    fn from(err: std::sync::mpsc::RecvError) -> Self {
        Error::Recv(err)
    }
}

impl From<std::sync::mpsc::SendError<std::vec::Vec<u8>>> for Error {
    fn from(err: std::sync::mpsc::SendError<std::vec::Vec<u8>>) -> Self {
        Error::Send(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

impl From<dcsjsonrpc_common::RpcError> for Error {
    fn from(err: dcsjsonrpc_common::RpcError) -> Self {
        Error::Rpc(err)
    }
}
