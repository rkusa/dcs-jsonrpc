use std::borrow::Cow;
use std::fmt;

use crate::jsonrpc::Client;
use crate::{Error, Identifier, Position};

#[derive(Clone)]
pub struct Unit {
    client: Client,
    id: Identifier,
}

impl Unit {
    pub(crate) fn new<I: Into<Identifier>>(client: Client, id: I) -> Self {
        Unit {
            client,
            id: id.into(),
        }
    }

    fn request<R>(&self, method: &str) -> Result<R, Error>
    where
        for<'de> R: serde::Deserialize<'de>,
    {
        self.client
            .request::<_, Option<R>>(method, Some(&self.id))?
            .ok_or_else(|| Error::GroupGone(self.id.clone()))
    }

    pub fn name(&self) -> Result<Cow<'_, str>, Error> {
        match self.id {
            Identifier::ID(_) => self.request("unitName").map(Cow::Owned),
            Identifier::Name(ref name) => Ok(Cow::Borrowed(name)),
        }
    }

    pub fn position(&self) -> Result<Position, Error> {
        self.request("unitPosition")
    }
}

pub struct UnitIterator {
    pub(crate) client: Client,
    pub(crate) unit_names: Vec<String>,
}

impl fmt::Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unit {{ id: {} }}", self.id)
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unit {}", self.id)
    }
}

impl Iterator for UnitIterator {
    type Item = Unit;

    fn next(&mut self) -> Option<Self::Item> {
        self.unit_names
            .pop()
            .map(|name| Unit::new(self.client.clone(), name))
    }
}
