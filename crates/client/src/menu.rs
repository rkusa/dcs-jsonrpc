use crate::jsonrpc::Client;
use crate::{Coalition, Error};
use serde_json::Value;

pub struct MenuEntry {
    client: Client,
    path: Value,
}

pub struct GroupMenuEntry {
    client: Client,
    group_id: usize,
    path: Value,
}

pub struct CoalitionMenuEntry {
    client: Client,
    coalition: Coalition,
    path: Value,
}

pub struct SubMenu<C>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de>,
{
    client: Client,
    path: Value,
    mark: std::marker::PhantomData<C>,
}

pub struct GroupSubMenu<C>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de>,
{
    client: Client,
    group_id: usize,
    path: Value,
    mark: std::marker::PhantomData<C>,
}

pub struct CoalitionSubMenu<C>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de>,
{
    client: Client,
    coalition: Coalition,
    path: Value,
    mark: std::marker::PhantomData<C>,
}

impl MenuEntry {
    pub fn remove(self) -> Result<(), Error> {
        remove_entry(&self.client, self.path)
    }
}

impl GroupMenuEntry {
    pub fn remove(self) -> Result<(), Error> {
        remove_group_entry(&self.client, self.group_id, self.path)
    }
}

impl CoalitionMenuEntry {
    pub fn remove(self) -> Result<(), Error> {
        remove_coalition_entry(&self.client, self.coalition, self.path)
    }
}

impl<C> SubMenu<C>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de>,
{
    pub fn add_submenu(&self, name: &str) -> Result<SubMenu<C>, Error> {
        add_submenu(&self.client, name, Some(&self.path))
    }

    pub fn add_command(&self, name: &str, command: C) -> Result<MenuEntry, Error> {
        add_command(&self.client, name, Some(&self.path), command)
    }

    pub fn remove(self) -> Result<(), Error> {
        remove_entry(&self.client, self.path)
    }
}

impl<C> GroupSubMenu<C>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de>,
{
    pub fn add_submenu(&self, name: &str) -> Result<GroupSubMenu<C>, Error> {
        add_group_submenu(&self.client, self.group_id, name, Some(&self.path))
    }

    pub fn add_command(&self, name: &str, command: C) -> Result<GroupMenuEntry, Error> {
        add_group_command(&self.client, self.group_id, name, Some(&self.path), command)
    }

    pub fn remove(self) -> Result<(), Error> {
        remove_group_entry(&self.client, self.group_id, self.path)
    }
}

impl<C> CoalitionSubMenu<C>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de>,
{
    pub fn add_submenu(&self, name: &str) -> Result<CoalitionSubMenu<C>, Error> {
        add_coalition_submenu(&self.client, self.coalition, name, Some(&self.path))
    }

    pub fn add_command(&self, name: &str, command: C) -> Result<CoalitionMenuEntry, Error> {
        add_coalition_command(
            &self.client,
            self.coalition,
            name,
            Some(&self.path),
            command,
        )
    }

    pub fn remove(self) -> Result<(), Error> {
        remove_coalition_entry(&self.client, self.coalition, self.path)
    }
}

pub(crate) fn add_submenu<C>(
    client: &Client,
    name: &str,
    parent: Option<&Value>,
) -> Result<SubMenu<C>, Error>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de>,
{
    #[derive(Serialize)]
    struct Params<'a> {
        name: &'a str,
        path: Option<&'a Value>,
    }

    let path: Value = client.request("addGroupSubMenu", Some(Params { name, path: parent }))?;

    Ok(SubMenu {
        client: client.clone(),
        path,
        mark: std::marker::PhantomData,
    })
}

pub(crate) fn add_group_submenu<C>(
    client: &Client,
    group_id: usize,
    name: &str,
    parent: Option<&Value>,
) -> Result<GroupSubMenu<C>, Error>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de>,
{
    #[derive(Serialize)]
    struct Params<'a> {
        #[serde(rename = "groupID")]
        group_id: usize,
        name: &'a str,
        path: Option<&'a Value>,
    }

    let path: Value = client.request(
        "addGroupSubMenu",
        Some(Params {
            group_id,
            name,
            path: parent,
        }),
    )?;

    Ok(GroupSubMenu {
        client: client.clone(),
        group_id,
        path,
        mark: std::marker::PhantomData,
    })
}

pub(crate) fn add_coalition_submenu<C>(
    client: &Client,
    coalition: Coalition,
    name: &str,
    parent: Option<&Value>,
) -> Result<CoalitionSubMenu<C>, Error>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de>,
{
    #[derive(Serialize)]
    struct Params<'a> {
        coalition: Coalition,
        name: &'a str,
        path: Option<&'a Value>,
    }

    let path: Value = client.request(
        "addCoalitionSubMenu",
        Some(Params {
            coalition,
            name,
            path: parent,
        }),
    )?;

    Ok(CoalitionSubMenu {
        client: client.clone(),
        coalition,
        path,
        mark: std::marker::PhantomData,
    })
}

pub(crate) fn add_command<C>(
    client: &Client,
    name: &str,
    parent: Option<&Value>,
    command: C,
) -> Result<MenuEntry, Error>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de>,
{
    #[derive(Serialize)]
    struct Params<'a, C>
    where
        C: serde::Serialize,
    {
        name: &'a str,
        path: Option<&'a Value>,
        command: C,
    }

    let path: Value = client.request(
        "addCommand",
        Some(Params {
            name,
            path: parent,
            command,
        }),
    )?;
    Ok(MenuEntry {
        client: client.clone(),
        path,
    })
}

pub(crate) fn add_group_command<C>(
    client: &Client,
    group_id: usize,
    name: &str,
    parent: Option<&Value>,
    command: C,
) -> Result<GroupMenuEntry, Error>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de>,
{
    #[derive(Serialize)]
    struct Params<'a, C>
    where
        C: serde::Serialize,
    {
        #[serde(rename = "groupID")]
        group_id: usize,
        name: &'a str,
        path: Option<&'a Value>,
        command: C,
    }

    let path: Value = client.request(
        "addGroupCommand",
        Some(Params {
            group_id,
            name,
            path: parent,
            command,
        }),
    )?;
    Ok(GroupMenuEntry {
        client: client.clone(),
        group_id,
        path,
    })
}

pub(crate) fn add_coalition_command<C>(
    client: &Client,
    coalition: Coalition,
    name: &str,
    parent: Option<&Value>,
    command: C,
) -> Result<CoalitionMenuEntry, Error>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de>,
{
    #[derive(Serialize)]
    struct Params<'a, C>
    where
        C: serde::Serialize,
    {
        coalition: Coalition,
        name: &'a str,
        path: Option<&'a Value>,
        command: C,
    }

    let path: Value = client.request(
        "addCoalitionCommand",
        Some(Params {
            coalition,
            name,
            path: parent,
            command,
        }),
    )?;
    Ok(CoalitionMenuEntry {
        client: client.clone(),
        coalition,
        path,
    })
}

pub(crate) fn remove_entry(client: &Client, path: Value) -> Result<(), Error> {
    #[derive(Serialize)]
    struct Params {
        path: Value,
    }

    client.notification("removeEntry", Some(Params { path }))
}

pub(crate) fn remove_group_entry(
    client: &Client,
    group_id: usize,
    path: Value,
) -> Result<(), Error> {
    #[derive(Serialize)]
    struct Params {
        #[serde(rename = "groupID")]
        group_id: usize,
        path: Value,
    }

    client.notification("removeGroupEntry", Some(Params { group_id, path }))
}

pub(crate) fn remove_coalition_entry(
    client: &Client,
    coalition: Coalition,
    path: Value,
) -> Result<(), Error> {
    #[derive(Serialize)]
    struct Params {
        coalition: Coalition,
        path: Value,
    }

    client.notification("removeGroupEntry", Some(Params { coalition, path }))
}
