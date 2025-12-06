//! Data models for MISP entities (events, attributes, galaxies, etc).

use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize};

fn flexible_bool_required<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum FlexBool {
        Bool(bool),
        String(String),
        Int(i64),
    }

    match FlexBool::deserialize(deserializer)? {
        FlexBool::Bool(b) => Ok(b),
        FlexBool::String(s) => Ok(s == "1" || s.eq_ignore_ascii_case("true")),
        FlexBool::Int(i) => Ok(i != 0),
    }
}

fn string_to_option_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Deserialize::deserialize(deserializer)?;
    Ok(opt.map(|s| s == "1" || s.eq_ignore_ascii_case("true")))
}

fn flexible_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum FlexBool {
        Bool(bool),
        String(String),
        Int(i64),
    }

    let opt: Option<FlexBool> = Deserialize::deserialize(deserializer)?;
    Ok(opt.map(|v| match v {
        FlexBool::Bool(b) => b,
        FlexBool::String(s) => s == "1" || s.eq_ignore_ascii_case("true"),
        FlexBool::Int(i) => i != 0,
    }))
}

fn string_or_int<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(i64),
    }

    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::String(s) => Ok(s),
        StringOrInt::Int(i) => Ok(i.to_string()),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "StringOrU8")]
pub enum ThreatLevel {
    High,
    Medium,
    Low,
    Undefined,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrU8 {
    String(String),
    Int(u8),
}

impl From<StringOrU8> for ThreatLevel {
    fn from(v: StringOrU8) -> Self {
        let n = match v {
            StringOrU8::String(s) => s.parse().unwrap_or(4),
            StringOrU8::Int(n) => n,
        };
        match n {
            1 => ThreatLevel::High,
            2 => ThreatLevel::Medium,
            3 => ThreatLevel::Low,
            _ => ThreatLevel::Undefined,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "StringOrU8")]
pub enum AnalysisLevel {
    Initial,
    Ongoing,
    Complete,
}

impl From<StringOrU8> for AnalysisLevel {
    fn from(v: StringOrU8) -> Self {
        let n = match v {
            StringOrU8::String(s) => s.parse().unwrap_or(0),
            StringOrU8::Int(n) => n,
        };
        match n {
            0 => AnalysisLevel::Initial,
            1 => AnalysisLevel::Ongoing,
            _ => AnalysisLevel::Complete,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "StringOrU8")]
pub enum Distribution {
    Organisation,
    Community,
    Connected,
    All,
    SharingGroup,
    Inherit,
}

impl From<StringOrU8> for Distribution {
    fn from(v: StringOrU8) -> Self {
        let n = match v {
            StringOrU8::String(s) => s.parse().unwrap_or(0),
            StringOrU8::Int(n) => n,
        };
        match n {
            0 => Distribution::Organisation,
            1 => Distribution::Community,
            2 => Distribution::Connected,
            3 => Distribution::All,
            4 => Distribution::SharingGroup,
            _ => Distribution::Inherit,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    #[serde(deserialize_with = "string_or_int")]
    pub id: String,
    pub uuid: String,
    pub info: String,
    #[serde(default, deserialize_with = "string_or_int")]
    pub org_id: String,
    #[serde(default, deserialize_with = "string_or_int")]
    pub orgc_id: String,
    #[serde(default)]
    pub date: Option<NaiveDate>,
    pub threat_level_id: ThreatLevel,
    pub analysis: AnalysisLevel,
    pub distribution: Distribution,
    #[serde(deserialize_with = "flexible_bool_required")]
    pub published: bool,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(default)]
    pub publish_timestamp: Option<String>,
    #[serde(default, rename = "Attribute")]
    pub attributes: Vec<Attribute>,
    #[serde(default, rename = "Object")]
    pub objects: Vec<MispObject>,
    #[serde(default, rename = "Tag")]
    pub tags: Vec<Tag>,
    #[serde(default, rename = "Galaxy")]
    pub galaxies: Vec<Galaxy>,
    #[serde(default, rename = "Org")]
    pub org: Option<Organisation>,
    #[serde(default, rename = "Orgc")]
    pub orgc: Option<Organisation>,
    #[serde(default)]
    pub attribute_count: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    #[serde(deserialize_with = "string_or_int")]
    pub id: String,
    pub uuid: String,
    #[serde(deserialize_with = "string_or_int")]
    pub event_id: String,
    pub category: String,
    #[serde(rename = "type")]
    pub attr_type: String,
    pub value: String,
    #[serde(deserialize_with = "flexible_bool_required")]
    pub to_ids: bool,
    pub distribution: Distribution,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(default, rename = "Tag")]
    pub tags: Vec<Tag>,
    #[serde(default, rename = "Galaxy")]
    pub galaxies: Vec<Galaxy>,
    #[serde(default, rename = "Sighting")]
    pub sightings: Vec<Sighting>,
    #[serde(default)]
    pub first_seen: Option<String>,
    #[serde(default)]
    pub last_seen: Option<String>,
    #[serde(default, deserialize_with = "flexible_bool")]
    pub deleted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MispObject {
    #[serde(deserialize_with = "string_or_int")]
    pub id: String,
    pub uuid: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub template_uuid: Option<String>,
    #[serde(default)]
    pub template_version: Option<String>,
    #[serde(deserialize_with = "string_or_int")]
    pub event_id: String,
    pub distribution: Distribution,
    #[serde(default, rename = "Attribute")]
    pub attributes: Vec<Attribute>,
    #[serde(default, rename = "ObjectReference")]
    pub references: Vec<ObjectReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectReference {
    #[serde(deserialize_with = "string_or_int")]
    pub id: String,
    pub uuid: String,
    #[serde(deserialize_with = "string_or_int")]
    pub object_id: String,
    #[serde(deserialize_with = "string_or_int")]
    pub referenced_id: String,
    #[serde(default)]
    pub referenced_uuid: Option<String>,
    #[serde(default)]
    pub referenced_type: Option<String>,
    #[serde(default)]
    pub relationship_type: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    #[serde(deserialize_with = "string_or_int")]
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub colour: Option<String>,
    #[serde(default, deserialize_with = "string_to_option_bool")]
    pub exportable: Option<bool>,
    #[serde(default)]
    pub numerical_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Galaxy {
    #[serde(deserialize_with = "string_or_int")]
    pub id: String,
    pub uuid: String,
    pub name: String,
    #[serde(rename = "type")]
    pub galaxy_type: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default, rename = "GalaxyCluster")]
    pub clusters: Vec<GalaxyCluster>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalaxyCluster {
    #[serde(deserialize_with = "string_or_int")]
    pub id: String,
    pub uuid: String,
    pub value: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default, rename = "GalaxyElement")]
    pub elements: Vec<GalaxyElement>,
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
    #[serde(default)]
    pub tag_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalaxyElement {
    #[serde(deserialize_with = "string_or_int")]
    pub id: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sighting {
    #[serde(deserialize_with = "string_or_int")]
    pub id: String,
    #[serde(deserialize_with = "string_or_int")]
    pub attribute_id: String,
    #[serde(deserialize_with = "string_or_int")]
    pub event_id: String,
    #[serde(default)]
    pub org_id: Option<String>,
    #[serde(default)]
    pub date_sighting: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default, rename = "type")]
    pub sighting_type: Option<String>,
    #[serde(default)]
    pub uuid: Option<String>,
    #[serde(default, rename = "Organisation")]
    pub organisation: Option<Organisation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organisation {
    #[serde(deserialize_with = "string_or_int")]
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warninglist {
    #[serde(deserialize_with = "string_or_int")]
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub list_type: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default, deserialize_with = "flexible_bool")]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarninglistCheckResult {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub matched: bool,
    #[serde(default)]
    pub warninglists: Vec<WarninglistMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarninglistMatch {
    #[serde(deserialize_with = "string_or_int")]
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub matched: Option<String>,
}
