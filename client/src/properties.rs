//! Client methods for working with VCO's system properties.

use std::collections::BTreeMap as Map;

use api_v1::date_time::DateTime;
//use api_v1::property::SystemProperty;
use api_v1::Number;

use crate::client::*;
use crate::error::*;

#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum PropertyValue {
    String(String),
    Number(Number),
    Boolean(bool),
    Json(String),
    Date(DateTime),
    Datetime(DateTime),
}

pub struct Property {
    pub name: String,
    pub value: PropertyValue,
    pub default_value: PropertyValue,
    pub is_read_only: bool,
    pub is_password: bool,
    pub description: String,
}

impl Property {
    // pub fn to_system_property(&self, id: Option<Integer>) -> SystemProperty {
    //     todo!()
    // }
}

/// Extending Client with methods for handling VCO properties.
impl Client {
    /// Gets the system properties.
    pub async fn get_system_properties(
        &self,
    ) -> Result<Vec<GetSystemPropertiesResultItem>, ClientError> {
        let resp = self
            .post_without_payload("systemProperty/getSystemProperties")
            .await?;
        Ok(serde_json::de::from_str(&resp).map_err(ClientError::Json)?)
    }

    /// Gets the system properties, converting the result to a mapping by property name.
    ///
    pub async fn get_system_properties_map(
        &self,
    ) -> Result<Map<String, GetSystemPropertiesResultItem>, ClientError> {
        Ok(self
            .get_system_properties()
            .await?
            .into_iter()
            .map(|item| (item.property.name.clone(), item))
            .collect())
    }

    // pub async fn get_system_property(&self, property_name: &str) -> Result<String, ClientError> {
    //     todo!()
    // }
    //
    // pub async fn set_system_property(&self, property: Property) -> Result<(), ClientError> {
    //     todo!()
    // }
}
