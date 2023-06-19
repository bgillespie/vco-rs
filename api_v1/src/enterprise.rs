use serde::{Deserialize, Serialize};

use crate::common::{BastionState, EndpointPkiMode};
use crate::date_time::DateTime;
use crate::tinyint::TinyInt;
use crate::{Double, Integer};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Enterprise {
    pub id: Integer,
    pub created: DateTime,
    pub network_id: Integer,
    pub gateway_pool_id: Integer,
    pub alerts_enabled: TinyInt,
    pub operator_alerts_enabled: TinyInt,
    pub endpoint_pki_mode: EndpointPkiMode,
    pub name: String,
    pub domain: Option<String>,
    pub prefix: Option<String>,
    // #[serde(with = "serde_logical_id")]
    pub logical_id: String,
    pub account_number: String,
    pub description: Option<String>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_mobile: Option<String>,
    pub contact_email: Option<String>,
    pub street_address: Option<String>,
    pub street_address2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub lat: Double,
    pub lon: Double,
    pub timezone: String,
    pub locale: String,
    pub modified: DateTime,
    pub bastion_state: BastionState,
}
