//! Low-level HTTP client for MISP API requests.

use reqwest::{header, Client, Method};
use serde_json::Value;
use std::time::Duration;
use tracing::{debug, error};

use crate::error::MispError;

#[derive(Clone)]
pub struct MispClient {
    base_url: String,
    api_key: String,
    http: Client,
}

impl MispClient {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>, verify_ssl: bool) -> Self {
        let base_url = base_url.into().trim_end_matches('/').to_string();

        let http = Client::builder()
            .danger_accept_invalid_certs(!verify_ssl)
            .timeout(Duration::from_secs(30))
            .build()
            .expect("failed to create HTTP client");

        Self {
            base_url,
            api_key: api_key.into(),
            http,
        }
    }

    pub fn with_timeout(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
        verify_ssl: bool,
        timeout: Duration,
    ) -> Self {
        let base_url = base_url.into().trim_end_matches('/').to_string();

        let http = Client::builder()
            .danger_accept_invalid_certs(!verify_ssl)
            .timeout(timeout)
            .build()
            .expect("failed to create HTTP client");

        Self {
            base_url,
            api_key: api_key.into(),
            http,
        }
    }

    pub async fn get(&self, endpoint: &str) -> Result<Value, MispError> {
        self.request(Method::GET, endpoint, None).await
    }

    pub async fn post(&self, endpoint: &str, body: Option<Value>) -> Result<Value, MispError> {
        self.request(Method::POST, endpoint, body).await
    }

    pub async fn delete(&self, endpoint: &str) -> Result<Value, MispError> {
        self.request(Method::DELETE, endpoint, None).await
    }

    async fn request(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<Value>,
    ) -> Result<Value, MispError> {
        let url = format!("{}{}", self.base_url, endpoint);
        debug!(%method, %url, "MISP API request");

        let mut req = self
            .http
            .request(method, &url)
            .header(header::AUTHORIZATION, &self.api_key)
            .header(header::ACCEPT, "application/json")
            .header(header::CONTENT_TYPE, "application/json");

        if let Some(json) = body {
            req = req.json(&json);
        }

        let resp = req.send().await?;
        let status = resp.status();

        if status == 401 || status == 403 {
            return Err(MispError::Authentication(format!(
                "API key rejected ({})",
                status
            )));
        }

        let text = resp.text().await?;
        debug!(response_len = text.len(), "MISP API response received");

        if !status.is_success() {
            error!(%url, %status, "MISP API error");
            return Err(MispError::Api {
                status: status.as_u16(),
                message: text,
            });
        }

        serde_json::from_str(&text).map_err(|e| {
            error!(%url, "Failed to parse MISP response");
            MispError::Parse(e)
        })
    }
}

impl std::fmt::Debug for MispClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MispClient")
            .field("base_url", &self.base_url)
            .field("api_key", &"[redacted]")
            .finish()
    }
}
