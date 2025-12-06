//! MISP sighting operations for tracking IOC observations.

use serde_json::{json, Value};
use tracing::debug;

use crate::client::MispClient;
use crate::error::MispError;
use crate::models::Sighting;

#[derive(Clone)]
pub struct SightingsClient {
    client: MispClient,
}

impl SightingsClient {
    pub fn new(client: MispClient) -> Self {
        Self { client }
    }

    pub async fn list_for_attribute(&self, attribute_id: &str) -> Result<Vec<Sighting>, MispError> {
        debug!(%attribute_id, "Fetching sightings for attribute");
        let resp = self
            .client
            .get(&format!(
                "/sightings/listSightings/{}/attribute",
                attribute_id
            ))
            .await?;
        parse_sightings_list(resp)
    }

    pub async fn list_for_event(&self, event_id: &str) -> Result<Vec<Sighting>, MispError> {
        debug!(%event_id, "Fetching sightings for event");
        let resp = self
            .client
            .get(&format!("/sightings/listSightings/{}/event", event_id))
            .await?;
        parse_sightings_list(resp)
    }

    pub async fn search(&self, query: SightingSearchQuery) -> Result<Vec<Sighting>, MispError> {
        debug!("Searching sightings");
        let resp = self
            .client
            .post("/sightings/restSearch", Some(query.to_json()))
            .await?;
        parse_sightings_search(resp)
    }

    pub async fn search_by_value(&self, value: &str) -> Result<Vec<Sighting>, MispError> {
        self.search(SightingSearchQuery::new().value(value)).await
    }

    pub async fn count_for_value(&self, value: &str) -> Result<SightingCount, MispError> {
        let sightings = self.search_by_value(value).await?;

        let mut positive = 0u64;
        let mut negative = 0u64;
        let mut expiration = 0u64;

        for s in &sightings {
            match s.sighting_type.as_deref() {
                Some("0") | None => positive += 1,
                Some("1") => negative += 1,
                Some("2") => expiration += 1,
                _ => positive += 1,
            }
        }

        let first_seen = sightings
            .iter()
            .filter_map(|s| s.date_sighting.as_ref())
            .min()
            .cloned();

        let last_seen = sightings
            .iter()
            .filter_map(|s| s.date_sighting.as_ref())
            .max()
            .cloned();

        Ok(SightingCount {
            total: sightings.len() as u64,
            positive,
            negative,
            expiration,
            first_seen,
            last_seen,
        })
    }

    pub async fn get_timeline(
        &self,
        value: &str,
        limit: Option<u32>,
    ) -> Result<Vec<SightingEntry>, MispError> {
        let mut query = SightingSearchQuery::new().value(value);
        if let Some(l) = limit {
            query = query.limit(l);
        }

        let sightings = self.search(query).await?;

        let mut entries: Vec<SightingEntry> = sightings
            .into_iter()
            .map(|s| SightingEntry {
                id: s.id,
                timestamp: s.date_sighting,
                source: s.source,
                org_name: s.organisation.map(|o| o.name),
                sighting_type: match s.sighting_type.as_deref() {
                    Some("0") | None => SightingType::Positive,
                    Some("1") => SightingType::Negative,
                    Some("2") => SightingType::Expiration,
                    _ => SightingType::Positive,
                },
            })
            .collect();

        entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(entries)
    }
}

impl std::fmt::Debug for SightingsClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SightingsClient").finish()
    }
}

#[derive(Debug, Default, Clone)]
pub struct SightingSearchQuery {
    pub value: Option<String>,
    pub uuid: Option<String>,
    pub attr_type: Option<String>,
    pub source: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub last: Option<String>,
    pub limit: Option<u32>,
    pub include_attribute: Option<bool>,
    pub include_event: Option<bool>,
}

impl SightingSearchQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }

    pub fn uuid(mut self, u: impl Into<String>) -> Self {
        self.uuid = Some(u.into());
        self
    }

    pub fn attr_type(mut self, t: impl Into<String>) -> Self {
        self.attr_type = Some(t.into());
        self
    }

    pub fn source(mut self, s: impl Into<String>) -> Self {
        self.source = Some(s.into());
        self
    }

    pub fn from_date(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    pub fn to_date(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    pub fn last(mut self, duration: impl Into<String>) -> Self {
        self.last = Some(duration.into());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn include_attribute(mut self) -> Self {
        self.include_attribute = Some(true);
        self
    }

    pub fn include_event(mut self) -> Self {
        self.include_event = Some(true);
        self
    }

    fn to_json(&self) -> Value {
        let mut obj = serde_json::Map::new();
        if let Some(ref v) = self.value {
            obj.insert("value".into(), json!(v));
        }
        if let Some(ref v) = self.uuid {
            obj.insert("uuid".into(), json!(v));
        }
        if let Some(ref v) = self.attr_type {
            obj.insert("type".into(), json!(v));
        }
        if let Some(ref v) = self.source {
            obj.insert("source".into(), json!(v));
        }
        if let Some(ref v) = self.from {
            obj.insert("from".into(), json!(v));
        }
        if let Some(ref v) = self.to {
            obj.insert("to".into(), json!(v));
        }
        if let Some(ref v) = self.last {
            obj.insert("last".into(), json!(v));
        }
        if let Some(v) = self.limit {
            obj.insert("limit".into(), json!(v));
        }
        if let Some(v) = self.include_attribute {
            obj.insert("includeAttribute".into(), json!(v));
        }
        if let Some(v) = self.include_event {
            obj.insert("includeEvent".into(), json!(v));
        }
        Value::Object(obj)
    }
}

#[derive(Debug, Clone)]
pub struct SightingCount {
    pub total: u64,
    pub positive: u64,
    pub negative: u64,
    pub expiration: u64,
    pub first_seen: Option<String>,
    pub last_seen: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SightingType {
    Positive,
    Negative,
    Expiration,
}

#[derive(Debug, Clone)]
pub struct SightingEntry {
    pub id: String,
    pub timestamp: Option<String>,
    pub source: Option<String>,
    pub org_name: Option<String>,
    pub sighting_type: SightingType,
}

fn parse_sightings_list(resp: Value) -> Result<Vec<Sighting>, MispError> {
    if let Some(arr) = resp.as_array() {
        let sightings: Result<Vec<Sighting>, _> = arr
            .iter()
            .filter_map(|v| v.get("Sighting"))
            .map(|s| serde_json::from_value(s.clone()))
            .collect();
        return sightings.map_err(MispError::Parse);
    }
    if resp.is_object() && resp.get("Sighting").is_none() {
        return Ok(Vec::new());
    }
    Err(MispError::InvalidResponse(
        "unexpected sightings format".into(),
    ))
}

fn parse_sightings_search(resp: Value) -> Result<Vec<Sighting>, MispError> {
    if let Some(response) = resp.get("response") {
        if let Some(arr) = response.as_array() {
            let sightings: Result<Vec<Sighting>, _> = arr
                .iter()
                .filter_map(|v| v.get("Sighting"))
                .map(|s| serde_json::from_value(s.clone()))
                .collect();
            return sightings.map_err(MispError::Parse);
        }
    }
    parse_sightings_list(resp)
}
