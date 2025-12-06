//! MISP event operations and search queries.

use serde_json::{json, Value};
use tracing::debug;

use crate::client::MispClient;
use crate::error::MispError;
use crate::models::{Attribute, Event, Galaxy, Tag};

#[derive(Clone)]
pub struct EventsClient {
    client: MispClient,
}

impl EventsClient {
    pub fn new(client: MispClient) -> Self {
        Self { client }
    }

    pub async fn get(&self, id: &str) -> Result<Event, MispError> {
        debug!(%id, "Fetching event");
        let resp = self.client.get(&format!("/events/view/{}", id)).await?;
        parse_event_response(resp)
    }

    pub async fn get_by_uuid(&self, uuid: &str) -> Result<Event, MispError> {
        self.get(uuid).await
    }

    pub async fn index(&self, params: Option<EventIndexParams>) -> Result<Vec<Event>, MispError> {
        debug!("Listing events");
        let body = params.map(|p| p.to_json());
        let resp = self.client.post("/events/index", body).await?;
        parse_events_list(resp)
    }

    pub async fn search(&self, query: EventSearchQuery) -> Result<Vec<Event>, MispError> {
        debug!("Searching events");
        let resp = self
            .client
            .post("/events/restSearch", Some(query.to_json()))
            .await?;
        parse_rest_search_events(resp)
    }

    pub async fn get_tags(&self, event_id: &str) -> Result<Vec<Tag>, MispError> {
        let event = self.get(event_id).await?;
        Ok(event.tags)
    }

    pub async fn get_attributes(&self, event_id: &str) -> Result<Vec<Attribute>, MispError> {
        let event = self.get(event_id).await?;
        Ok(event.attributes)
    }

    pub async fn get_galaxies(&self, event_id: &str) -> Result<Vec<Galaxy>, MispError> {
        let event = self.get(event_id).await?;
        Ok(event.galaxies)
    }
}

impl std::fmt::Debug for EventsClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventsClient").finish()
    }
}

#[derive(Debug, Default, Clone)]
pub struct EventIndexParams {
    pub limit: Option<u32>,
    pub page: Option<u32>,
    pub sort: Option<String>,
    pub direction: Option<String>,
    pub minimal: Option<bool>,
    pub published: Option<bool>,
    pub org: Option<String>,
    pub tags: Option<Vec<String>>,
    pub from: Option<String>,
    pub to: Option<String>,
}

impl EventIndexParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn published(mut self, published: bool) -> Self {
        self.published = Some(published);
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
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

    fn to_json(&self) -> Value {
        let mut obj = serde_json::Map::new();
        if let Some(v) = self.limit {
            obj.insert("limit".into(), json!(v));
        }
        if let Some(v) = self.page {
            obj.insert("page".into(), json!(v));
        }
        if let Some(ref v) = self.sort {
            obj.insert("sort".into(), json!(v));
        }
        if let Some(ref v) = self.direction {
            obj.insert("direction".into(), json!(v));
        }
        if let Some(v) = self.minimal {
            obj.insert("minimal".into(), json!(v));
        }
        if let Some(v) = self.published {
            obj.insert("published".into(), json!(v));
        }
        if let Some(ref v) = self.org {
            obj.insert("org".into(), json!(v));
        }
        if let Some(ref v) = self.tags {
            obj.insert("tags".into(), json!(v));
        }
        if let Some(ref v) = self.from {
            obj.insert("from".into(), json!(v));
        }
        if let Some(ref v) = self.to {
            obj.insert("to".into(), json!(v));
        }
        Value::Object(obj)
    }
}

#[derive(Debug, Default, Clone)]
pub struct EventSearchQuery {
    pub value: Option<String>,
    pub event_id: Option<String>,
    pub tags: Option<Vec<String>>,
    pub not_tags: Option<Vec<String>>,
    pub org: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub last: Option<String>,
    pub published: Option<bool>,
    pub threat_level: Option<Vec<u8>>,
    pub analysis: Option<Vec<u8>>,
    pub include_attribute: Option<bool>,
    pub include_galaxy: Option<bool>,
    pub limit: Option<u32>,
    pub page: Option<u32>,
}

impl EventSearchQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }

    pub fn event_id(mut self, id: impl Into<String>) -> Self {
        self.event_id = Some(id.into());
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    pub fn exclude_tags(mut self, tags: Vec<String>) -> Self {
        self.not_tags = Some(tags);
        self
    }

    pub fn org(mut self, org: impl Into<String>) -> Self {
        self.org = Some(org.into());
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

    pub fn published(mut self, published: bool) -> Self {
        self.published = Some(published);
        self
    }

    pub fn threat_level(mut self, levels: Vec<u8>) -> Self {
        self.threat_level = Some(levels);
        self
    }

    pub fn include_attributes(mut self) -> Self {
        self.include_attribute = Some(true);
        self
    }

    pub fn include_galaxies(mut self) -> Self {
        self.include_galaxy = Some(true);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    fn to_json(&self) -> Value {
        let mut obj = serde_json::Map::new();
        if let Some(ref v) = self.value {
            obj.insert("value".into(), json!(v));
        }
        if let Some(ref v) = self.event_id {
            obj.insert("eventid".into(), json!(v));
        }
        if let Some(ref v) = self.tags {
            obj.insert("tags".into(), json!(v));
        }
        if let Some(ref v) = self.not_tags {
            obj.insert("not_tags".into(), json!(v));
        }
        if let Some(ref v) = self.org {
            obj.insert("org".into(), json!(v));
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
        if let Some(v) = self.published {
            obj.insert("published".into(), json!(v));
        }
        if let Some(ref v) = self.threat_level {
            obj.insert("threat_level_id".into(), json!(v));
        }
        if let Some(ref v) = self.analysis {
            obj.insert("analysis".into(), json!(v));
        }
        if let Some(v) = self.include_attribute {
            obj.insert("includeAttribute".into(), json!(v));
        }
        if let Some(v) = self.include_galaxy {
            obj.insert("includeGalaxy".into(), json!(v));
        }
        if let Some(v) = self.limit {
            obj.insert("limit".into(), json!(v));
        }
        if let Some(v) = self.page {
            obj.insert("page".into(), json!(v));
        }
        Value::Object(obj)
    }
}

fn parse_event_response(resp: Value) -> Result<Event, MispError> {
    if let Some(event) = resp.get("Event") {
        return serde_json::from_value(event.clone()).map_err(MispError::Parse);
    }
    Err(MispError::InvalidResponse("missing Event wrapper".into()))
}

fn parse_events_list(resp: Value) -> Result<Vec<Event>, MispError> {
    if let Some(arr) = resp.as_array() {
        let events: Result<Vec<Event>, _> = arr
            .iter()
            .map(|v| {
                let event_val = v.get("Event").unwrap_or(v);
                serde_json::from_value(event_val.clone())
            })
            .collect();
        return events.map_err(MispError::Parse);
    }
    Err(MispError::InvalidResponse("expected array".into()))
}

fn parse_rest_search_events(resp: Value) -> Result<Vec<Event>, MispError> {
    if let Some(response) = resp.get("response") {
        if let Some(arr) = response.as_array() {
            let events: Result<Vec<Event>, _> = arr
                .iter()
                .map(|v| {
                    let event_val = v.get("Event").unwrap_or(v);
                    serde_json::from_value(event_val.clone())
                })
                .collect();
            return events.map_err(MispError::Parse);
        }
    }
    if resp.as_array().is_some() {
        return parse_events_list(resp);
    }
    Err(MispError::InvalidResponse(
        "unexpected response format".into(),
    ))
}
