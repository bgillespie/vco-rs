use serde::{Deserialize, Serialize};

//use crate::date_time::{Interval, DateTime};
use crate::common::{ActivationState, EndpointPkiMode, ServiceState};
use crate::tinyint::TinyInt;
use crate::Integer;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum BastionPromotedState {
    Unconfigured,
    StageRequested,
    UnstageRequested,
    Staged,
    Unstaged,
    PromotionRequested,
    PromotionPending,
    Promoted,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum EdgeState {
    NeverActivated,
    Degraded,
    Offline,
    Disabled,
    Expired,
    Connected,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum HaState {
    Unconfigured,
    PendingInit,
    PendingConfirmation,
    PendingConfirmed,
    PendingDissociation,
    Ready,
    Failed,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EdgeObject {
    pub activation_key: String,
    pub activation_key_expires: String,
    pub activation_state: ActivationState,
    pub activation_time: String,
    pub alerts_enabled: TinyInt,
    pub bastion_state: BastionPromotedState,
    pub build_number: String,
    pub created: String,
    pub custom_info: String,
    pub description: String,
    pub device_family: String,
    pub device_id: String,
    pub dns_name: String,
    pub edge_state: EdgeState,
    pub edge_state_time: String,
    pub endpoint_pki_mode: EndpointPkiMode,
    pub enterprise_id: Integer,
    pub factory_software_version: String,
    pub factory_build_number: String,
    pub ha_last_contact: String,
    pub ha_previous_state: HaState,
    pub ha_serial_number: String,
    pub ha_state: HaState,
    pub id: Integer,
    pub is_live: Integer,
    pub last_contact: String,
    // #[serde(with = "serde_logical_id")]
    pub logical_id: String,
    pub model_number: String,
    pub modified: String,
    pub name: String,
    pub operator_alerts_enabled: TinyInt,
    pub self_mac_address: String,
    pub serial_number: String,
    pub service_state: ServiceState,
    pub service_up_since: String,
    pub site_id: Integer,
    pub software_updated: String,
    pub software_version: String,
    pub system_up_since: String,
}
