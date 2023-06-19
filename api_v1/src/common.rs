//! These are data structures that are used inside multiple API modules.
use serde::{Deserialize, Serialize};

/// `ServiceState` is used in `edge` and `gateway`.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum ServiceState {
    InService,
    OutOfService,
    PendingService,
    Quiesced,
}

/// `ActivationState` is used in `edge` and `gateway`.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum ActivationState {
    Unassigned,
    Pending,
    Activated,
    ReactivationPending,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum TcpOrUdp {
    Tcp,
    Udp,
}

/// `BastionState` is used in `enterprise` and `gateway`.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum BastionState {
    Unconfigured,
    StageRequested,
    UnstageRequested,
    Staged,
    Unstaged,
}

/// `EndpointPkiMode` is used in `edge`, `enterprise` and `gateway`.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum EndpointPkiMode {
    CertificateDisabled,
    CertificateOptional,
    CertificateRequired,
}
