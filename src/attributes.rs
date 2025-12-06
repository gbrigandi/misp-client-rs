//! MISP attribute operations and search queries.

use serde_json::{json, Value};
use tracing::debug;

use crate::client::MispClient;
use crate::error::MispError;
use crate::models::Attribute;

#[derive(Clone)]
pub struct AttributesClient {
    client: MispClient,
}

impl AttributesClient {
    pub fn new(client: MispClient) -> Self {
        Self { client }
    }

    pub async fn get(&self, id: &str) -> Result<Attribute, MispError> {
        debug!(%id, "Fetching attribute");
        let resp = self.client.get(&format!("/attributes/view/{}", id)).await?;
        parse_attribute_response(resp)
    }

    pub async fn search(&self, query: AttributeSearchQuery) -> Result<Vec<Attribute>, MispError> {
        debug!("Searching attributes");
        let resp = self
            .client
            .post("/attributes/restSearch", Some(query.to_json()))
            .await?;
        parse_rest_search_attributes(resp)
    }

    pub async fn search_by_value(&self, value: &str) -> Result<Vec<Attribute>, MispError> {
        self.search(AttributeSearchQuery::new().value(value)).await
    }

    pub async fn search_by_type(&self, attr_type: &str) -> Result<Vec<Attribute>, MispError> {
        self.search(AttributeSearchQuery::new().attr_type(attr_type))
            .await
    }

    pub async fn describe_types(&self) -> Result<AttributeTypes, MispError> {
        debug!("Fetching attribute types");
        let resp = self.client.get("/attributes/describeTypes").await?;
        parse_describe_types(resp)
    }

    pub async fn correlations(&self, value: &str) -> Result<Vec<AttributeCorrelation>, MispError> {
        debug!(%value, "Searching correlations");
        let query = AttributeSearchQuery::new()
            .value(value)
            .include_correlations(true);
        let attrs = self.search(query).await?;

        let mut correlations = Vec::new();
        for attr in attrs {
            correlations.push(AttributeCorrelation {
                attribute_id: attr.id.clone(),
                attribute_uuid: attr.uuid.clone(),
                event_id: attr.event_id.clone(),
                value: attr.value.clone(),
                attr_type: attr.attr_type.clone(),
                category: attr.category.clone(),
            });
        }
        Ok(correlations)
    }
}

impl std::fmt::Debug for AttributesClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AttributesClient").finish()
    }
}

#[derive(Debug, Default, Clone)]
pub struct AttributeSearchQuery {
    pub value: Option<String>,
    pub attr_type: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub not_tags: Option<Vec<String>>,
    pub org: Option<String>,
    pub event_id: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub last: Option<String>,
    pub to_ids: Option<bool>,
    pub published: Option<bool>,
    pub deleted: Option<bool>,
    pub include_event_uuid: Option<bool>,
    pub include_correlations: Option<bool>,
    pub include_sightings: Option<bool>,
    pub include_galaxy: Option<bool>,
    pub limit: Option<u32>,
    pub page: Option<u32>,
    pub timestamp: Option<String>,
}

impl AttributeSearchQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }

    pub fn attr_type(mut self, t: impl Into<String>) -> Self {
        self.attr_type = Some(t.into());
        self
    }

    pub fn category(mut self, c: impl Into<String>) -> Self {
        self.category = Some(c.into());
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

    pub fn event_id(mut self, id: impl Into<String>) -> Self {
        self.event_id = Some(id.into());
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

    pub fn to_ids(mut self, to_ids: bool) -> Self {
        self.to_ids = Some(to_ids);
        self
    }

    pub fn published(mut self, published: bool) -> Self {
        self.published = Some(published);
        self
    }

    pub fn include_deleted(mut self) -> Self {
        self.deleted = Some(true);
        self
    }

    pub fn include_event_uuid(mut self) -> Self {
        self.include_event_uuid = Some(true);
        self
    }

    pub fn include_correlations(mut self, include: bool) -> Self {
        self.include_correlations = Some(include);
        self
    }

    pub fn include_sightings(mut self) -> Self {
        self.include_sightings = Some(true);
        self
    }

    pub fn include_galaxy(mut self) -> Self {
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

    pub fn timestamp(mut self, ts: impl Into<String>) -> Self {
        self.timestamp = Some(ts.into());
        self
    }

    fn to_json(&self) -> Value {
        let mut obj = serde_json::Map::new();

        if let Some(ref v) = self.value {
            obj.insert("value".into(), json!(v));
        }
        if let Some(ref v) = self.attr_type {
            obj.insert("type".into(), json!(v));
        }
        if let Some(ref v) = self.category {
            obj.insert("category".into(), json!(v));
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
        if let Some(ref v) = self.event_id {
            obj.insert("eventid".into(), json!(v));
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
        if let Some(v) = self.to_ids {
            obj.insert("to_ids".into(), json!(v));
        }
        if let Some(v) = self.published {
            obj.insert("published".into(), json!(v));
        }
        if let Some(v) = self.deleted {
            obj.insert("deleted".into(), json!(v));
        }
        if let Some(v) = self.include_event_uuid {
            obj.insert("includeEventUuid".into(), json!(v));
        }
        if let Some(v) = self.include_correlations {
            obj.insert("includeCorrelations".into(), json!(v));
        }
        if let Some(v) = self.include_sightings {
            obj.insert("includeSightings".into(), json!(v));
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
        if let Some(ref v) = self.timestamp {
            obj.insert("timestamp".into(), json!(v));
        }

        Value::Object(obj)
    }
}

#[derive(Debug, Clone)]
pub struct AttributeTypes {
    pub types: Vec<String>,
    pub categories: Vec<String>,
    pub category_type_mappings: std::collections::HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct AttributeCorrelation {
    pub attribute_id: String,
    pub attribute_uuid: String,
    pub event_id: String,
    pub value: String,
    pub attr_type: String,
    pub category: String,
}

fn parse_attribute_response(resp: Value) -> Result<Attribute, MispError> {
    if let Some(attr) = resp.get("Attribute") {
        return serde_json::from_value(attr.clone()).map_err(MispError::Parse);
    }
    Err(MispError::InvalidResponse("missing Attribute wrapper".into()))
}

fn parse_rest_search_attributes(resp: Value) -> Result<Vec<Attribute>, MispError> {
    if let Some(response) = resp.get("response") {
        if let Some(attr_wrapper) = response.get("Attribute") {
            if let Some(arr) = attr_wrapper.as_array() {
                return arr
                    .iter()
                    .map(|a| serde_json::from_value(a.clone()))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(MispError::Parse);
            }
        }
    }

    if let Some(response) = resp.get("response") {
        if let Some(arr) = response.as_array() {
            let attrs: Result<Vec<Attribute>, _> = arr
                .iter()
                .filter_map(|v| v.get("Attribute"))
                .map(|a| serde_json::from_value(a.clone()))
                .collect();
            return attrs.map_err(MispError::Parse);
        }
    }

    Err(MispError::InvalidResponse(
        "unexpected attribute response format".into(),
    ))
}

fn parse_describe_types(resp: Value) -> Result<AttributeTypes, MispError> {
    let result = resp.get("result").ok_or_else(|| {
        MispError::InvalidResponse("missing result in describeTypes".into())
    })?;

    let types = result
        .get("types")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(String::from)
                .collect()
        })
        .unwrap_or_default();

    let categories = result
        .get("categories")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(String::from)
                .collect()
        })
        .unwrap_or_default();

    let mut category_type_mappings = std::collections::HashMap::new();
    if let Some(mappings) = result.get("category_type_mappings").and_then(|v| v.as_object()) {
        for (cat, types_val) in mappings {
            if let Some(arr) = types_val.as_array() {
                let type_list: Vec<String> = arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect();
                category_type_mappings.insert(cat.clone(), type_list);
            }
        }
    }

    Ok(AttributeTypes {
        types,
        categories,
        category_type_mappings,
    })
}
