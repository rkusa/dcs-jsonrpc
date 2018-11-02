use std::borrow::Cow;
use std::fmt;

use crate::jsonrpc::Client;
use crate::{Error, Identifier};

/// Represents all objects placed on the map. Bridges, buildings, etc.
#[derive(Clone)]
pub struct Scenery {
    client: Client,
    id: Identifier,
}

impl Scenery {
    pub(crate) fn new<I: Into<Identifier>>(client: Client, id: I) -> Self {
        Scenery {
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

impl fmt::Debug for Scenery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scenery {{ id: {} }}", self.id)
    }
}

impl fmt::Display for Scenery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scenery {}", self.id)
    }
}
