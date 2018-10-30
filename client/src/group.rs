use crate::error::Error;
use crate::jsonrpc::Client;

pub struct Group {
    client: Client,
    name: String,
}

impl Group {
    pub(crate) fn new(client: Client, name: &str) -> Self {
        Group {
            client,
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn exists(&self) -> Result<bool, Error> {
        #[derive(Serialize)]
        struct Params<'a> {
            name: &'a str,
        }

        let exists: bool = self
            .client
            .request("group_isExist", Some(Params { name: &self.name }))?;

        Ok(exists)
    }
}
