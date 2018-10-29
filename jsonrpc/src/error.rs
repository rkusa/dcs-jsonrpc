use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    Lua(::hlua51::LuaError),
    Undefined(String),
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
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        use self::Error::*;

        match *self {
            Lua(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<::hlua51::LuaError> for Error {
    fn from(err: ::hlua51::LuaError) -> Self {
        Error::Lua(err)
    }
}
