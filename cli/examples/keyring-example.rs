use keyring::{Entry, Result};

fn main() -> Result<()> {
    let entry = Entry::new("my_service", "my_name")?;
    entry.set_password("topS3cr3tP4$$w0rd")?;

    let entry = Entry::new("my_service", "my_name")?;
    let password = entry.get_password()?;
    println!("My password is '{}'", password);

    entry.delete_password()?;
    Ok(())
}
