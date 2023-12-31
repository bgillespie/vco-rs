use anyhow::Result;

use client::client::Client as VcoClient;

/// List all the system properties as name/value pairs.
///
/// TODO consideration should be given to the other parameters of each property.
/// TODO currently the response is a newline-separated string. It should return
///      `Vec<(String, PropertyParams)>`.
///
/// If `filter` is specified, then only properties whose names start with the filter string will be
/// shown. If `show_passwords` is `false` then any properties marked as `isPassword` in the response
/// will be redacted.
pub(crate) async fn list(vco: &VcoClient, filter: &str, show_passwords: bool) -> Result<String> {
    let result = vco.get_system_properties().await?;
    let result = result
        .into_iter()
        .filter(|item| item.property.name.starts_with(filter))
        .map(|item| {
            format!(
                "{} => {}",
                item.property.name,
                if show_passwords || !item.property.is_password.0 {
                    &item.property.value
                } else {
                    "****"
                }
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    Ok(result)
}

// async fn get(vco: &VcoClient, property_name: &str) -> Result<String> {
//     todo!()
// }
//
// async fn set(vco: &VcoClient, property_name: &str, property_value: &str) -> Result<String> {
//     todo!()
// }
//
// async fn delete(vco: &VcoClient, property_name: &str, _: &str) -> Result<String> {
//     todo!()
// }
