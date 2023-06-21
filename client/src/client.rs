//! Client module to handle the interactions with VCO.
//!

// TODO we need a way for users to be able to specify relative or absolute datetimes.

use reqwest::header::HeaderMap;
use reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{ClientError, Error as ApiError};
pub use api_v1::error::SerdeError;
pub use api_v1::gateway::{GatewayMetric, NetworkGetNetworkGatewaysResultItem};
use api_v1::login::AuthObject;
pub use api_v1::property::GetSystemPropertiesResultItem;

/// Use `Client` to make calls to VCO's API. This is the main interface to the rest of the `client`
/// crate.
///
/// The intention with `Client` is to insulate callers from the vagaries of the different underlying
/// VCO APIs, where they have changed over time, presenting a predictable API.
pub struct Client {
    pub(crate) client: reqwest::Client,
    pub(crate) hostname: String,
    pub(crate) domain: String,
}

impl Client {
    //
    // LOGIN METHODS
    //

    /// Do an authenticate with username and password.
    ///
    /// This authentication method uses cookies from the get-go.
    pub async fn operator_login_password(
        fqdn: &str,
        username: &str,
        password: &str,
    ) -> Result<Self, ClientError> {
        let (hostname, domain) = fqdn_to_name_and_domain(fqdn)?;
        let default_headers = Self::common_client_headers(&hostname, &domain);
        let client_builder = Self::common_client_builder(default_headers);
        let req_client = client_builder
            .build()
            .map_err(ClientError::ReqwestClientCreate)?;

        let client = Self {
            client: req_client,
            hostname,
            domain,
        };

        // Build the request body.
        let auth_object = AuthObject::new(username.into(), password.into());

        // Do the actual login. The response body is empty so we just discard it.
        client
            .post_with_payload("login/operatorLogin", &auth_object)
            .await?;

        Ok(client)
    }

    /// Do token-based auth.
    pub async fn operator_login_token(fqdn: &str, token: &str) -> Result<Self, ClientError> {
        let (hostname, domain) = fqdn_to_name_and_domain(fqdn)?;
        // Set up default headers.
        let mut default_headers = Self::common_client_headers(&hostname, &domain);
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
            hostname,
            domain,
        })
    }

    //
    // REST API CALLS
    //

    /// The general-case POST request.
    /// Users should use the `post-with-payload` and `post-without-payload` calls instead.
    /// If the response is empty then it will try to vivify `T` from `null`.
    async fn post<'de, T>(
        &self,
        path: &str,
        payload: Option<&impl Serialize>,
    ) -> Result<T, ClientError>
    where
        T: serde::Deserialize<'de>,
    {
        // Start building a POST request.
        let mut resp_builder = self.client.post(self.rest_api_url(path));

        // Attach the payload if there is one.
        if let Some(payload) = payload {
            let raw = serde_json::to_string(&payload).map_err(ClientError::Json)?;
            resp_builder = resp_builder.body(raw);
        }

        // Send the request and await the response.
        // If we get an error before we get a response, surface it to the caller now.
        let resp = resp_builder.send().await.map_err(ClientError::Request)?;

        // Read the body text of the response.
        // NOTE: We're trusting VCO not to send back an unreasonably-sized body here.
        let text = resp.text().await.map_err(ClientError::Response)?;

        // If the response is empty, try to vivify T from `null`.
        if text.is_empty() {
            return Ok(T::deserialize(serde_json::Value::Null).map_err(ClientError::Json)?);
        }

        // Interpret the body of the response as JSON.
        let json: Value = serde_json::from_str(&text).map_err(ClientError::Json)?;

        // Check the response to see if it's an error and respond accordingly.
        if let Some(text) = Self::identify_error_body(&json) {
            Err(ClientError::Api(text.to_string()))
        } else {
            Ok(T::deserialize(json).map_err(ClientError::Json)?)
        }
    }

    pub(crate) async fn post_without_payload<'de, T>(&self, path: &str) -> Result<T, ClientError>
    where
        T: serde::Deserialize<'de>,
    {
        let payload: Option<&serde_json::Value> = None; // keep type-checking happy
        self.post(path, payload).await
    }

    pub(crate) async fn post_with_payload<'de, T>(
        &self,
        path: &str,
        payload: &impl Serialize,
    ) -> Result<T, ClientError>
    where
        T: Deserialize<'de>,
    {
        self.post(path, Some(payload)).await
    }

    //
    // UTILITY METHODS
    //

    /// Check if we've received an API V1-style error body.
    fn identify_error_body(json: &serde_json::Value) -> Option<String> {
        // Error is always contained in a mapping, or "object" in JSON parlance.
        if let serde_json::Value::Object(top_level) = json {
            // Does the mapping contain an "error" key?
            if let Some(error_value) = top_level.get("error") {
                // Yes so try to match it to a VCO API error.
                match <ApiError as Deserialize>::deserialize(error_value) {
                    Ok(api_error) => {
                        // It deserializes to an ApiError, so it must be a real error.
                        Some(format!("{} ({})", api_error.message, api_error.code))
                    }
                    Err(_) => {
                        // It doesn't deserialize as an `ApiError`, it must be something else.
                        None
                    }
                }
            } else {
                // There's no "error" key in the top-level map, so this can't be an error.
                None
            }
        } else {
            // The top level isn't a mapping, which all error messages are.
            None
        }
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

fn fqdn_to_name_and_domain(vco_fqdn: &str) -> Result<(String, String), ClientError> {
    let parts = vco_fqdn.splitn(2, ".").collect::<Vec<&str>>();
    if parts.len() != 2 {
        return Err(ClientError::BadVcoFqdn(
            "Bad FQDN format, expected at least one dot in name, not \"{vco_fqdn}\".".to_string(),
        ));
    }
    let vco_name = parts[0].to_string().to_lowercase();
    let vco_domain = parts[1].to_string().to_lowercase();
    if !vco_name.starts_with("vco") {
        return Err(ClientError::BadVcoFqdn(
            "VCO name must start with \"vco\"; got \"{vco_fqdn}\".".to_string(),
        ));
    }
    Ok((vco_name, vco_domain))
}
