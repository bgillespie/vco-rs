//! Client methods for working with Gateways (VCG).

// TODO don't "go around" the client module.
use api_v1::date_time::*;
use api_v1::gateway::*;
use api_v1::Integer;

use crate::client::*;
use crate::error::*;

impl Client {
    /// Get the status metrics for a VCG.
    /// TODO accept and return types not coming directly from whichever underlying API crate.
    pub async fn get_gateway_status_metrics(
        &self,
        gateway_id: Integer,
        start: &DateTime,
        end: Option<&DateTime>,
        metrics: &[GatewayMetric],
    ) -> Result<String, ClientError> {
        let body = GetGatewayStatusMetrics {
            gateway_id,
            interval: Interval {
                end: end.map(DateTime::clone),
                start: start.clone(),
            },
            metrics: metrics.into(),
        };
        let body = serde_json::ser::to_string(&body).expect("Couldn't JSON serialize body");
        println!("{}", serde_json::to_string_pretty(&body).unwrap());

        let resp = self
            .post_with_payload("/metrics/getGatewayStatusMetrics", &body)
            .await?;
        Ok(resp)
    }

    /// Get a list of all the network gateways, AKA "VCGs".
    /// TODO return some type not coming directly from whichever underlying API crate.
    /// TODO `/network/getNetworkGateways` allow passing in `with` params:
    ///      `{"with":["site","roles","pools","dataCenters","certificates","enterprises",
    ///                "handOffEdges","enterpriseAssociationCounts"]}`
    pub async fn get_network_gateways(
        &self,
    ) -> Result<Vec<NetworkGetNetworkGatewaysResultItem>, ClientError> {
        let resp = self
            .post_without_payload("network/getNetworkGateways")
            .await?;
        Ok(resp)
    }
}
