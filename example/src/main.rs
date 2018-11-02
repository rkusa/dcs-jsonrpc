use dcsjsonrpc_client::{Client, Error};

fn main() -> Result<(), Error> {
    let client = Client::connect("127.0.0.1:7777")?;
    let group1 = client.group("group1");
    if group1.exists()? {
        println!("Group {} does exist", group1.name()?);
    } else {
        println!("Group {} does not exist", group1.name()?);
    }

    println!("Listening to events:");
    for event in client.events()? {
        println!("{}", event);
    }

    Ok(())
}
