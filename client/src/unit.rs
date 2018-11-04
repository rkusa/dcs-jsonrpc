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

    pub fn name(&self) -> Result<Cow<'_, str>, Error> {
        match self.id {
            Identifier::ID(_) => self
                .client
                .request("unitName", Some(&self.id))
                .map(Cow::Owned),
            Identifier::Name(ref name) => Ok(Cow::Borrowed(name)),
        }
    }

    pub fn position(&self) -> Result<Option<Position>, Error> {
        self.client.request("unitPosition", Some(&self.id))
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
