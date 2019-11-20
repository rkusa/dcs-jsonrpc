use std::{error, fmt};

use lua51 as ffi;

#[derive(Debug)]
pub enum Error {
    ArgumentType(String, String),
    SerializeResult(serde_json::Error, String),
    SerializeBroadcast(serde_json::Error, String),
    Io(std::io::Error),
    Utf8(std::str::Utf8Error),
    StackSize(usize, usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Error::*;
        use std::error::Error;

        match self {
            ArgumentType(name, kind) => {
                write!(f, "Expected argument {} to be of type {}", name, kind)?
            }
            SerializeResult(_, ref res) => write!(f, "Error serializing result: {}", res)?,
            SerializeBroadcast(_, ref res) => {
                write!(f, "Error serializing broadcast payload: {}", res)?
            }
            StackSize(expected, got) => {
                write!(f, "Expected a Lua stack size of {}, got {}", expected, got)?
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
            ArgumentType(_, _) => "Wrong argument type",
            SerializeResult(_, _) => "Error serializing RPC result",
            SerializeBroadcast(_, _) => "Error serializing broadcast payload",
            Io(_) => "Error in TCP connection",
            Utf8(_) => "UTF8 error in cstr",
            StackSize(_, _) => "Unexpected Lua stack size",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        use self::Error::*;

        match *self {
            SerializeResult(ref err, _) => Some(err),
            SerializeBroadcast(ref err, _) => Some(err),
            Io(ref err) => Some(err),
            Utf8(ref err) => Some(err),
            _ => None,
        }
    }
}

pub fn assert_stack_size(l: *mut ffi::lua_State, expected: usize) -> Result<(), Error> {
    let curr = unsafe { ffi::lua_gettop(l) } as usize;
    if curr != expected {
        Err(Error::StackSize(expected, curr))
    } else {
        Ok(())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Error::Utf8(err)
    }
}
