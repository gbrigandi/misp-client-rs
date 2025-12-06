//! MISP warninglist operations for false positive reduction.

use serde_json::{json, Value};
use tracing::debug;

use crate::client::MispClient;
use crate::error::MispError;
use crate::models::{Warninglist, WarninglistCheckResult, WarninglistMatch};

#[derive(Clone)]
pub struct WarninglistsClient {
    client: MispClient,
}

impl WarninglistsClient {
    pub fn new(client: MispClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<Warninglist>, MispError> {
        debug!("Listing warninglists");
        let resp = self.client.get("/warninglists/index").await?;
        parse_warninglists(resp)
    }

    pub async fn list_enabled(&self) -> Result<Vec<Warninglist>, MispError> {
        let all = self.list().await?;
        Ok(all.into_iter().filter(|w| w.enabled == Some(true)).collect())
    }

    pub async fn get(&self, id: &str) -> Result<Warninglist, MispError> {
        debug!(%id, "Fetching warninglist");
        let resp = self
            .client
            .get(&format!("/warninglists/view/{}", id))
            .await?;
        parse_warninglist(resp)
    }

    pub async fn check_value(&self, value: &str) -> Result<WarninglistCheckResult, MispError> {
        debug!(%value, "Checking value against warninglists");
        let body = json!({ "value": value });
        let resp = self
            .client
            .post("/warninglists/checkValue", Some(body))
            .await?;
        parse_check_result(resp, value)
    }

    pub async fn check_values(&self, values: &[&str]) -> Result<Vec<WarninglistCheckResult>, MispError> {
        debug!(count = values.len(), "Checking values against warninglists");
        let body = json!({ "value": values });
        let resp = self
            .client
            .post("/warninglists/checkValue", Some(body))
            .await?;
        parse_check_results(resp, values)
    }

    pub async fn is_whitelisted(&self, value: &str) -> Result<bool, MispError> {
        let result = self.check_value(value).await?;
        Ok(result.matched)
    }

    pub async fn get_matching_lists(&self, value: &str) -> Result<Vec<WarninglistMatch>, MispError> {
        let result = self.check_value(value).await?;
        Ok(result.warninglists)
    }
}

impl std::fmt::Debug for WarninglistsClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WarninglistsClient").finish()
    }
}

fn parse_warninglists(resp: Value) -> Result<Vec<Warninglist>, MispError> {
    if let Some(wls) = resp.get("Warninglists") {
        if let Some(arr) = wls.as_array() {
            let lists: Result<Vec<Warninglist>, _> = arr
                .iter()
                .filter_map(|v| v.get("Warninglist"))
                .map(|w| serde_json::from_value(w.clone()))
                .collect();
            return lists.map_err(MispError::Parse);
        }
    }
    if let Some(arr) = resp.as_array() {
        let lists: Result<Vec<Warninglist>, _> = arr
            .iter()
            .filter_map(|v| v.get("Warninglist"))
            .map(|w| serde_json::from_value(w.clone()))
            .collect();
        return lists.map_err(MispError::Parse);
    }
    Err(MispError::InvalidResponse("unexpected warninglists format".into()))
}

fn parse_warninglist(resp: Value) -> Result<Warninglist, MispError> {
    if let Some(wl) = resp.get("Warninglist") {
        return serde_json::from_value(wl.clone()).map_err(MispError::Parse);
    }
    Err(MispError::InvalidResponse("missing Warninglist wrapper".into()))
}

fn parse_check_result(resp: Value, value: &str) -> Result<WarninglistCheckResult, MispError> {
    let matched = resp
        .get(value)
        .and_then(|v| v.as_array())
        .map(|arr| !arr.is_empty())
        .unwrap_or(false);

    let warninglists = if matched {
        resp.get(value)
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        let id = item.get("id")?.as_str()?.to_string();
                        let name = item.get("name")?.as_str()?.to_string();
                        let matched_entry = item.get("matched").and_then(|m| m.as_str()).map(String::from);
                        Some(WarninglistMatch {
                            id,
                            name,
                            matched: matched_entry,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    Ok(WarninglistCheckResult {
        value: value.to_string(),
        matched,
        warninglists,
    })
}

fn parse_check_results(resp: Value, values: &[&str]) -> Result<Vec<WarninglistCheckResult>, MispError> {
    values
        .iter()
        .map(|v| parse_check_result(resp.clone(), v))
        .collect()
}
