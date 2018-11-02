use std::borrow::Cow;
use std::fmt;

use crate::jsonrpc::Client;
use crate::{Error, Identifier};

#[derive(Clone)]
pub struct Airbase {
    client: Client,
    id: Identifier,
}

impl Airbase {
    pub(crate) fn new<I: Into<Identifier>>(client: Client, id: I) -> Self {
        Airbase {
            client,
            id: id.into(),
        }
    }

    pub fn name(&self) -> Result<Cow<'_, str>, Error> {
        match self.id {
            Identifier::ID(_) => self
                .client
                .request("unitName", Some(&self.id))
                .map(Cow::Owned),
            Identifier::Name(ref name) => Ok(Cow::Borrowed(name)),
        }
    }
}

impl fmt::Debug for Airbase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Airbase {{ id: {} }}", self.id)
    }
}

impl fmt::Display for Airbase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Airbase {}", self.id)
    }
}
