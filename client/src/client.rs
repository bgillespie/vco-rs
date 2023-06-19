//! Client module to handle the interactions with VCO.
//!

pub use api_v1::error::SerdeError;
pub use api_v1::gateway::{GatewayMetric, NetworkGetNetworkGatewaysResultItem};
use api_v1::login::AuthObject;
pub use api_v1::property::GetSystemPropertiesResultItem;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::header::HeaderMap;
use reqwest::ClientBuilder;
use serde::Serialize;

use crate::error::ClientError;

/// Regex to match an incoming error response.
static ERROR_MATCH: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^\s*\{\s*"error":"#).unwrap());

/// A Client to make calls to VCO's API.
pub struct Client {
    pub(crate) client: reqwest::Client,
    pub(crate) hostname: String,
    pub(crate) domain: String,
}

impl Client {
    //
    // LOGIN METHODS
    //

    /// Do an authenticate with username and password for cookie-based auth.
    pub async fn operator_login_password(
        hostname: &str,
        domain: &str,
        username: &str,
        password: &str,
    ) -> Result<Self, ClientError> {
        let default_headers = Self::common_client_headers(hostname, domain);
        let client_builder = Self::common_client_builder(default_headers);
        let client = client_builder
            .build()
            .map_err(ClientError::ReqwestClientCreate)?;

        // Build the request body.
        let auth_object = AuthObject::new(username.into(), password.into());
        let body = serde_json::to_string(&auth_object).map_err(ClientError::Json)?;

        // Make the call and wait for the response.
        client
            .post(format!(
                "https://{hostname}.{domain}/login/doOperatorLogin.html"
            ))
            .body(body)
            .send()
            .await
            .map_err(ClientError::Request)?;

        // TODO check and handle login failure

        Ok(Self {
            client,
            hostname: hostname.into(),
            domain: domain.into(),
        })
    }

    /// Do token-based auth.
    pub async fn operator_login_token(
        hostname: &str,
        domain: &str,
        token: &str,
    ) -> Result<Self, ClientError> {
        // Set up default headers.
        let mut default_headers = Self::common_client_headers(hostname, domain);
        default_headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Token {token}").parse().unwrap(),
        );

        // Set up client builder.
        let client_builder = Self::common_client_builder(default_headers);

        // Build client.
        let client = client_builder
            .build()
            .map_err(ClientError::ReqwestClientCreate)?;

        // TODO do something to confirm good token

        Ok(Self {
            client,
            hostname: hostname.into(),
            domain: domain.into(),
        })
    }

    //
    // REST API CALLS
    //

    pub(crate) async fn post_without_payload(&self, path: &str) -> Result<String, ClientError> {
        let resp = self
            .client
            .post(self.rest_api_url(path))
            .send()
            .await
            .map_err(ClientError::Request)?;

        // We're trusting VCO not to send back an unreasonably-sized body here.
        let text = resp.text().await.map_err(ClientError::Response)?;
        if Self::identify_error_body(&text) {
            Err(ClientError::Api(text))
        } else {
            Ok(text)
        }
    }

    pub(crate) async fn post_with_payload(
        &self,
        path: &str,
        payload: &impl Serialize,
    ) -> Result<String, ClientError> {
        let resp = self
            .client
            .post(self.rest_api_url(path))
            .body(serde_json::to_string(&payload).map_err(ClientError::Json)?)
            .send()
            .await
            .map_err(ClientError::Request)?;

        // We're trusting VCO not to send back an unreasonably-sized body here.
        let text = resp.text().await.map_err(ClientError::Response)?;
        if Self::identify_error_body(&text) {
            Err(ClientError::Api(text))
        } else {
            Ok(text)
        }
    }

    //
    // UTILITY METHODS
    //

    /// Check if we've received an API V1-style error body.
    fn identify_error_body(body: &str) -> bool {
        ERROR_MATCH.is_match(body)
    }

    /// Generate the headers map common to all calls.
    fn common_client_headers(hostname: &str, domain: &str) -> HeaderMap {
        let fqdn = format!("{hostname}.{domain}");

        // Set up default headers.
        let mut default_headers = HeaderMap::new();
        default_headers.insert(reqwest::header::HOST, fqdn.parse().unwrap());
        default_headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        default_headers
    }

    /// Generate the reqwests ClientBuilder used for all Client creates.
    fn common_client_builder(default_headers: HeaderMap) -> ClientBuilder {
        ClientBuilder::new()
            .default_headers(default_headers)
            .user_agent("vco-rs")
            .cookie_store(true)
    }

    /// Generate the FQDN from the hostname and domain.
    #[inline]
    fn fqdn(&self) -> String {
        [self.hostname.as_str(), self.domain.as_str()].join(".")
    }

    /// Generate a REST API URL.
    #[inline]
    pub(crate) fn rest_api_url(&self, path: &str) -> String {
        format!(
            "https://{fqdn}/{api_base}/{path}",
            fqdn = self.fqdn(),
            api_base = api_v1::API_BASE
        )
    }
}
