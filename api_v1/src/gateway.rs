//! Gateway (VCG) related data structures.

use std::net::{Ipv4Addr, Ipv6Addr};

// use mac_address::MacAddress;
use serde::{Deserialize, Serialize};

use crate::date_time::{DateTime, Interval};
use crate::network_address::Address;
use crate::tinyint::TinyInt;
use crate::{Double, Integer, Map, Number, Set};

use crate::common::{ActivationState, BastionState, EndpointPkiMode, ServiceState, TcpOrUdp};
use crate::edge::EdgeObject;
use crate::enterprise::Enterprise;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum GatewayMetric {
    TunnelCount,
    MemoryPct,
    FlowCount,
    CpuPct,
    HandoffQueueDrops,
    ConnectedEdges,
    TunnelCountV6,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct GatewayMetrics(Set<GatewayMetric>);

impl FromIterator<GatewayMetric> for GatewayMetrics {
    fn from_iter<T: IntoIterator<Item = GatewayMetric>>(iter: T) -> Self {
        GatewayMetrics(Set::from_iter(iter))
    }
}

impl From<&[GatewayMetric]> for GatewayMetrics {
    fn from(value: &[GatewayMetric]) -> Self {
        value.iter().map(GatewayMetric::clone).collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetGatewayStatusMetrics {
    pub gateway_id: Integer,
    pub interval: Interval,
    pub metrics: GatewayMetrics,
}

#[cfg(test)]
mod test_get_gateway_status_metrics {
    use super::*;
    use crate::date_time::DateTime;

    #[test]
    fn test_get_gateway_metrics_roundtrip() {
        let mut metrics_set = Set::new();
        metrics_set.insert(GatewayMetric::TunnelCount);
        metrics_set.insert(GatewayMetric::FlowCount);
        let metrics = GatewayMetrics(metrics_set);

        let interval = Interval {
            end: None,
            start: DateTime::from_rfc3339("2023-01-02T03:04:05+05:30").unwrap(),
        };

        let ggm = GetGatewayStatusMetrics {
            gateway_id: 1,
            interval: interval.clone(),
            metrics: metrics.clone(),
        };

        // Test conversion roundtrip.
        let s = serde_json::ser::to_string(&ggm).unwrap();
        println!("{}", s);
        let d: GetGatewayStatusMetrics = serde_json::de::from_str(&s).unwrap();
        println!("{:?}", d);

        assert_eq!(d.metrics, metrics);
        assert_eq!(d.interval, interval);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum GatewayState {
    NeverActivated,
    Degraded,
    Quiesced,
    Disabled,
    OutOfService,
    Connected,
    Offline,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GatewayCertificate {
    pub id: Integer,
    pub created: DateTime,
    pub csr_id: Integer,
    pub gateway_id: Integer,
    pub network_id: Integer,
    pub certificate: String,
    pub serial_number: String,
    pub subject_key_id: String,
    pub finger_print: String,
    pub finger_print_256: String,
    pub valid_from: DateTime,
    pub valid_to: DateTime,
}

//
// Gateway Enterprise
//

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GatewayEnterpriseAssoc {
    #[serde(flatten)]
    pub enterprise: Enterprise,
    pub enterprise_id: Integer,
    pub enterprise_object_id: Option<Integer>,
    pub enterprise_object_name: Option<String>,
    pub enterprise_object_type: Option<String>,
    pub edge_id: Option<Integer>,
    pub edge_name: Option<String>,
    // #[serde(with = "serde_logical_id")]
    pub edge_logical_id: Option<String>,
    pub gateway_type: GatewayType,
    pub pinned: Integer,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum GatewayType {
    Other,
    Super,
    Datacenter,
    Handoff,
    SuperAlt,
    Primary,
    Secondary,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    pub id: Integer,
    pub created: DateTime,
    pub name: Option<String>,
    // #[serde(with = "serde_logical_id")]
    pub logical_id: String,
    pub contact_name: String,
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
    pub shipping_same_as_location: TinyInt,
    pub shipping_contact_name: Option<String>,
    pub shipping_address: Option<String>,
    pub shipping_address2: Option<String>,
    pub shipping_city: Option<String>,
    pub shipping_state: Option<String>,
    pub shipping_country: Option<String>,
    pub shipping_postal_code: Option<String>,
    pub modified: DateTime,
}

//
// Gateway Pool
//

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GatewayPool {
    pub id: Integer,
    pub network_id: Integer,
    pub enterprise_proxy_id: Option<Integer>,
    pub created: DateTime,
    pub name: String,
    pub description: Option<String>,
    // #[serde(with = "serde_logical_id")]
    pub logical_id: String,
    pub is_default: TinyInt,
    pub ip_v4_enabled: TinyInt,
    pub ip_v6_enabled: TinyInt,
    pub hand_off_type: GatewayHandoffType,
    pub modified: DateTime,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GatewayGatewayPool {
    #[serde(flatten)]
    pub gateway_pool: GatewayPool,
    pub gateway_pool_assoc_id: Integer,
    pub gateway_id: Integer,
}

//
// Gateway utilization
//

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UtilizationDetail {
    pub load: Number,
    pub overall: Number,
    pub cpu: Number,
    pub memory: Number,
}

//
// Gateway Handoff
//

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GatewayHandoffDetailSubnet {
    pub name: String,
    pub route_cost: u8,
    pub cidr_ip: Address<Ipv4Addr>,
    pub cidr_prefix: u8,
    pub encrypt: bool,
    pub hand_off_type: GatewayHandoffDetailSubnetHandoffType,
    // Support unknown fields
    // #[serde(flatten)]
    // unhandled_fields: Map<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum GatewayHandoffDetailSubnetHandoffType {
    Nat,
    Vlan,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum GatewayHandoffType {
    None,
    Allow,
    Only,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GatewayHandoffDetailIcmpProbe {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probe_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_tag: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_tag: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ip: Option<Address<Ipv4Addr>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_seconds: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<Integer>,
    // Support unknown fields
    // #[serde(flatten)]
    // unhandled_fields: Map<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GatewayHandoffDetailIcmpResponder {
    pub enabled: bool,
    pub ip_address: Address<Ipv4Addr>,
    pub mode: String,
    // Support unknown fields
    // #[serde(flatten)]
    // unhandled_fields: Map<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GatewayHandoffDetail {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,
    pub subnets: Vec<GatewayHandoffDetailSubnet>,
    pub icmp_probe: GatewayHandoffDetailIcmpProbe,
    pub icmp_responder: GatewayHandoffDetailIcmpResponder,
    // Support unknown fields
    // #[serde(flatten)]
    // unhandled_fields: Map<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GatewayHandoffEdge {
    #[serde(flatten)]
    pub _edge_object: EdgeObject,
    pub edge_id: Integer,
    pub is_primary: Integer,
    pub pinned: Integer,
    // #[serde(with = "serde_logical_id")]
    pub enterprise_logical_id: String,
    pub enterprise_name: String,
    // Support unknown fields
    // #[serde(flatten)]
    // unhandled_fields: Map<String, serde_json::Value>,
}

//
// Gateway Role
//

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum GatewayRoleType {
    DataPlane,
    ControlPlane,
    VpnTunnel,
    OnPremise,
    Cde,
    Cws,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GatewayRole {
    created: DateTime,
    gateway_id: Integer,
    // TODO what does this property of gatewayRole mean?
    //      "x-alternate-name": "gatewayRoleProperty"
    gateway_role: GatewayRoleType,
    required: Integer,
    // Support unknown fields
    // #[serde(flatten)]
    // unhandled_fields: Map<String, serde_json::Value>,
}

//
// Gateway Syslog
//

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum SyslogLocalFacility {
    Local0,
    Local1,
    Local2,
    Local3,
    Local4,
    Local5,
    Local6,
    Local7,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum GatewaySyslogCollectorSeverity {
    Info,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GatewaySyslogCollectorSettings {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub protocol: Option<TcpOrUdp>,
    pub severity: Option<GatewaySyslogCollectorSeverity>,
    // Support unknown fields
    // #[serde(flatten)]
    // unhandled_fields: Map<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GatewaySyslogSettings {
    pub tag: String,
    pub facility_code: SyslogLocalFacility,
    pub collectors: Vec<GatewaySyslogCollectorSettings>,
    // Support unknown fields
    // #[serde(flatten)]
    // unhandled_fields: Map<String, serde_json::Value>,
}

//
// Gateway IPsec
//

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IpsecGatewayDetail {
    pub enabled: bool,
    pub strict_host_check: bool,
    pub strict_host_check_d_n: Option<String>,
    // Support unknown fields
    // #[serde(flatten)]
    // unhandled_fields: Map<String, serde_json::Value>,
}

/// Data structure returned by doing the `network/getNetworkGateways`, which is an array of these.
///
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkGetNetworkGatewaysResultItem {
    pub id: Integer,
    pub name: String,
    pub description: Option<String>,
    pub dns_name: Option<String>,
    pub created: DateTime,
    /// `logicalId` is like "gateway01234567-89ab-cdef-0123-456789abcdef"
    pub logical_id: String,
    pub network_id: Option<Integer>,
    pub enterprise_proxy_id: Option<Integer>,
    pub site_id: Integer,
    pub software_version: String,
    pub build_number: String,
    // Is `deviceId` sometimes a UUID and sometimes a MAC address?
    pub device_id: Option<String>,

    pub ip_address: Option<Address<Ipv4Addr>>,
    pub ip_v6_address: Option<Address<Ipv6Addr>>,

    pub last_contact: DateTime,
    pub modified: DateTime,
    pub service_up_since: DateTime,
    pub system_up_since: DateTime,

    pub activation_key: String,
    pub activation_state: ActivationState,
    pub activation_time: DateTime,

    pub gateway_state: GatewayState,
    pub bastion_state: BastionState,
    pub service_state: ServiceState,

    pub utilization: Number,
    pub utilization_detail: Option<UtilizationDetail>,

    pub endpoint_pki_mode: EndpointPkiMode,

    pub connected_edges: Integer,
    pub connected_edge_list: Option<Vec<Map<String, serde_json::Value>>>,

    pub hand_off_detail: Option<GatewayHandoffDetail>,

    pub alerts_enabled: Option<TinyInt>,
    pub ipsec_gateway_detail: Option<IpsecGatewayDetail>,

    pub is_load_balanced: TinyInt,

    pub private_ip_address: Option<Address<Ipv4Addr>>,

    //
    // Chosen in `with` parameter
    //
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<GatewayCertificate>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_centers: Option<Vec<serde_json::Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_associations: Option<Vec<GatewayEnterpriseAssoc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_association_count: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprises: Option<Vec<Enterprise>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_off_edges: Option<Vec<GatewayHandoffEdge>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pools: Option<Vec<GatewayGatewayPool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<GatewayRole>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub syslog: Option<GatewaySyslogCollectorSettings>,

    //
    // End `with` params
    //

    // pub data_center_vpn_states: Option<???>

    // Support unhandled fields
    #[serde(flatten)]
    unhandled_fields: Map<String, serde_json::Value>,
}

#[cfg(test)]
mod test_get_gateways {
    use super::*;
    use std::io::Read;

    fn load_test_data(filename: &str) -> String {
        // let test_data_path = std::env::var("TESTS_DIR")
        //     .expect("Need to set env-var for tests dir");
        let test_data_path = "/home/bjg/Work/vco-py/notebook";
        let mut buf = String::new();
        std::fs::File::open(format!("{test_data_path}/{filename}"))
            .expect(&format!("Couldn't load {test_data_path}"))
            .read_to_string(&mut buf)
            .expect("Some sort of read error");
        buf
    }

    #[test]
    fn test_load_get_gateways() {
        let src = load_test_data("real-vco-gateway-data.json");
        let gateways_data: Vec<NetworkGetNetworkGatewaysResultItem> =
            serde_json::de::from_str(&src).unwrap();
        println!(
            "private_ip_address {:?}",
            gateways_data[0].private_ip_address
        );
        println!("ip_address {:?}", gateways_data[0].ip_address);
        println!("ip_v6_address {:?}", gateways_data[0].ip_v6_address);
        println!("unhandled_fields {:?}", gateways_data[0].unhandled_fields);

        for item in gateways_data {
            println!("{:?}", item.utilization_detail)
        }
    }
}
