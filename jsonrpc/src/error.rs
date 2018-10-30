use std::{error, fmt};

type CallbackExecutionError =
    hlua51::LuaFunctionCallError<hlua51::TuplePushError<hlua51::Void, hlua51::Void>>;

#[derive(Debug)]
pub enum Error {
    Lua(::hlua51::LuaError),
    Undefined(String),
    SerializeResult(serde_json::Error, String),
    SerializeBroadcast(serde_json::Error, String),
    CallbackExecution(CallbackExecutionError),
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Error::*;
        use std::error::Error;

        match self {
            Undefined(key) => write!(
                f,
                "Error: Trying to access undefined lua global or table key: {}",
                key
            )?,
            SerializeResult(_, ref res) => write!(f, "Error serializing result: {}", res)?,
            SerializeBroadcast(_, ref res) => {
                write!(f, "Error serializing broadcast payload: {}", res)?
            }
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
            Lua(_) => "Lua error",
            Undefined(_) => "Trying to access lua gobal or table key that does not exist",
            SerializeResult(_, _) => "Error serializing RPC result",
            SerializeBroadcast(_, _) => "Error serializing broadcast payload",
            CallbackExecution(_) => "Error executing RPC callback",
            Io(_) => "Error in TCP connection",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        use self::Error::*;

        match *self {
            Lua(ref err) => Some(err),
            SerializeResult(ref err, _) => Some(err),
            SerializeBroadcast(ref err, _) => Some(err),
            Io(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<::hlua51::LuaError> for Error {
    fn from(err: ::hlua51::LuaError) -> Self {
        Error::Lua(err)
    }
}

impl From<CallbackExecutionError> for Error {
    fn from(err: CallbackExecutionError) -> Self {
        Error::CallbackExecution(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
