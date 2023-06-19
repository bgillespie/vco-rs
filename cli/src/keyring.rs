use anyhow::Result;
use keyring::Entry as KeyringEntry;

/// In order to support both username/password and username/token-based auth, the "user" for the
/// credential as stored in the keyring is the username prepended by the credential type, and
/// separated by a colon, e.g. "PASSWORD:alice@example.com"; "TOKEN:alice@example.com".
///
/// `credential_type` should be either "password" or "token". Case doesn't matter.
fn get_cred_name(credential_type: &str, username: &str) -> String {
    let credential_type = credential_type.trim().to_uppercase();
    if !["PASSWORD", "TOKEN"].contains(&credential_type.as_str()) {
        panic!(
            "Something bad happened, expected \"PASSWORD\" or \"TOKEN\", not \"{credential_type}\"."
        )
    }
    [credential_type, username.into()].join(":")
}

/// Read the credential -- whatever type it is -- from the command line.
fn get_secret_for_user(credential_type: &str, username: &str) -> Result<String> {
    let secret = rpassword::prompt_password(format!("Enter {credential_type} for {username} ... "))
        .map_err(|e| anyhow::format_err!("Error getting {credential_type}: {:?}", e))?;
    Ok(secret)
}

/// Create or update a keyring entry.
fn set_credential(vco_fqdn: &str, username: &str, credential_type: &str) -> Result<String> {
    let cred_name = get_cred_name(&credential_type, &username);
    let secret = get_secret_for_user(&credential_type, &username)?;
    let entry = KeyringEntry::new(&vco_fqdn, &cred_name)?;
    entry.set_password(&secret)?;
    Ok(format!(
        "Set {credential_type} for {username} on {vco_fqdn}."
    ))
}

/// Read the credential from the keyring.
fn get_credential(vco_fqdn: &str, username: &str, credential_type: &str) -> Result<String> {
    let cred_name = get_cred_name(&credential_type, &username);
    let entry = KeyringEntry::new(&vco_fqdn, &cred_name)?;
    Ok(entry.get_password()?)
}

/// Delete the credential from the keyring.
fn delete_credential(vco_fqdn: &str, username: &str, credential_type: &str) -> Result<String> {
    let cred_name = get_cred_name(&credential_type, &username);
    let entry = KeyringEntry::new(&vco_fqdn, &cred_name)?;
    entry.delete_password()?;
    Ok(format!(
        "Deleted {username}'s {credential_type} for {vco_fqdn} from keyring"
    ))
}

pub(crate) fn set_token(vco_fqdn: &str, username: &str) -> Result<String> {
    set_credential(vco_fqdn, username, "token")
}

pub(crate) fn set_password(vco_fqdn: &str, username: &str) -> Result<String> {
    set_credential(vco_fqdn, username, "password")
}

pub(crate) fn get_token(vco_fqdn: &str, username: &str) -> Result<String> {
    get_credential(vco_fqdn, username, "token")
}

pub(crate) fn get_password(vco_fqdn: &str, username: &str) -> Result<String> {
    get_credential(vco_fqdn, username, "password")
}

pub(crate) fn delete_token(vco_fqdn: &str, username: &str) -> Result<String> {
    delete_credential(vco_fqdn, username, "token")
}

pub(crate) fn delete_password(vco_fqdn: &str, username: &str) -> Result<String> {
    delete_credential(vco_fqdn, username, "password")
}
