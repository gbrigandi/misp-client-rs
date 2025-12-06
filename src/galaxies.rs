//! MISP galaxy and cluster operations (threat actors, MITRE ATT&CK, malware).

use serde_json::{json, Value};
use tracing::debug;

use crate::client::MispClient;
use crate::error::MispError;
use crate::models::{Galaxy, GalaxyCluster};

#[derive(Clone)]
pub struct GalaxiesClient {
    client: MispClient,
}

impl GalaxiesClient {
    pub fn new(client: MispClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<Galaxy>, MispError> {
        debug!("Listing galaxies");
        let resp = self.client.get("/galaxies").await?;
        parse_galaxies_list(resp)
    }

    pub async fn get(&self, id: &str) -> Result<Galaxy, MispError> {
        debug!(%id, "Fetching galaxy");
        let resp = self.client.get(&format!("/galaxies/view/{}", id)).await?;
        parse_galaxy_response(resp)
    }

    pub async fn get_cluster(&self, cluster_id: &str) -> Result<GalaxyCluster, MispError> {
        debug!(%cluster_id, "Fetching galaxy cluster");
        let resp = self
            .client
            .get(&format!("/galaxy_clusters/view/{}", cluster_id))
            .await?;
        parse_cluster_response(resp)
    }

    pub async fn search_clusters(
        &self,
        query: ClusterSearchQuery,
    ) -> Result<Vec<GalaxyCluster>, MispError> {
        debug!("Searching galaxy clusters");
        let resp = self
            .client
            .post("/galaxy_clusters/restSearch", Some(query.to_json()))
            .await?;
        parse_clusters_search(resp)
    }

    pub async fn search_clusters_by_value(&self, value: &str) -> Result<Vec<GalaxyCluster>, MispError> {
        self.search_clusters(ClusterSearchQuery::new().value(value))
            .await
    }

    pub async fn get_by_type(&self, galaxy_type: &str) -> Result<Option<Galaxy>, MispError> {
        let galaxies = self.list().await?;
        Ok(galaxies.into_iter().find(|g| g.galaxy_type == galaxy_type))
    }

    pub async fn get_mitre_attack(&self) -> Result<Option<Galaxy>, MispError> {
        self.get_by_type("mitre-attack-pattern").await
    }

    pub async fn get_threat_actors(&self) -> Result<Option<Galaxy>, MispError> {
        self.get_by_type("threat-actor").await
    }

    pub async fn get_malware(&self) -> Result<Option<Galaxy>, MispError> {
        self.get_by_type("malpedia").await
    }

    pub async fn search_threat_actor(&self, name: &str) -> Result<Vec<GalaxyCluster>, MispError> {
        self.search_clusters(
            ClusterSearchQuery::new()
                .value(name)
                .galaxy_type("threat-actor"),
        )
        .await
    }

    pub async fn search_attack_pattern(&self, name: &str) -> Result<Vec<GalaxyCluster>, MispError> {
        self.search_clusters(
            ClusterSearchQuery::new()
                .value(name)
                .galaxy_type("mitre-attack-pattern"),
        )
        .await
    }
}

impl std::fmt::Debug for GalaxiesClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GalaxiesClient").finish()
    }
}

#[derive(Debug, Default, Clone)]
pub struct ClusterSearchQuery {
    pub value: Option<String>,
    pub galaxy_type: Option<String>,
    pub tag_name: Option<String>,
    pub uuid: Option<String>,
    pub source: Option<String>,
}

impl ClusterSearchQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }

    pub fn galaxy_type(mut self, t: impl Into<String>) -> Self {
        self.galaxy_type = Some(t.into());
        self
    }

    pub fn tag_name(mut self, t: impl Into<String>) -> Self {
        self.tag_name = Some(t.into());
        self
    }

    pub fn uuid(mut self, u: impl Into<String>) -> Self {
        self.uuid = Some(u.into());
        self
    }

    pub fn source(mut self, s: impl Into<String>) -> Self {
        self.source = Some(s.into());
        self
    }

    fn to_json(&self) -> Value {
        let mut obj = serde_json::Map::new();
        if let Some(ref v) = self.value {
            obj.insert("value".into(), json!(v));
        }
        if let Some(ref v) = self.galaxy_type {
            obj.insert("type".into(), json!(v));
        }
        if let Some(ref v) = self.tag_name {
            obj.insert("tag_name".into(), json!(v));
        }
        if let Some(ref v) = self.uuid {
            obj.insert("uuid".into(), json!(v));
        }
        if let Some(ref v) = self.source {
            obj.insert("source".into(), json!(v));
        }
        Value::Object(obj)
    }
}

fn parse_galaxies_list(resp: Value) -> Result<Vec<Galaxy>, MispError> {
    if let Some(arr) = resp.as_array() {
        let galaxies: Result<Vec<Galaxy>, _> = arr
            .iter()
            .filter_map(|v| v.get("Galaxy"))
            .map(|g| serde_json::from_value(g.clone()))
            .collect();
        return galaxies.map_err(MispError::Parse);
    }
    Err(MispError::InvalidResponse("expected array of galaxies".into()))
}

fn parse_galaxy_response(resp: Value) -> Result<Galaxy, MispError> {
    if let Some(galaxy) = resp.get("Galaxy") {
        let mut g: Galaxy = serde_json::from_value(galaxy.clone()).map_err(MispError::Parse)?;
        if let Some(clusters) = resp.get("GalaxyCluster").and_then(|v| v.as_array()) {
            g.clusters = clusters
                .iter()
                .filter_map(|c| serde_json::from_value(c.clone()).ok())
                .collect();
        }
        return Ok(g);
    }
    Err(MispError::InvalidResponse("missing Galaxy wrapper".into()))
}

fn parse_cluster_response(resp: Value) -> Result<GalaxyCluster, MispError> {
    if let Some(cluster) = resp.get("GalaxyCluster") {
        return serde_json::from_value(cluster.clone()).map_err(MispError::Parse);
    }
    Err(MispError::InvalidResponse("missing GalaxyCluster wrapper".into()))
}

fn parse_clusters_search(resp: Value) -> Result<Vec<GalaxyCluster>, MispError> {
    if let Some(response) = resp.get("response") {
        if let Some(arr) = response.as_array() {
            let clusters: Result<Vec<GalaxyCluster>, _> = arr
                .iter()
                .filter_map(|v| v.get("GalaxyCluster"))
                .map(|c| serde_json::from_value(c.clone()))
                .collect();
            return clusters.map_err(MispError::Parse);
        }
    }
    if let Some(arr) = resp.as_array() {
        let clusters: Result<Vec<GalaxyCluster>, _> = arr
            .iter()
            .filter_map(|v| v.get("GalaxyCluster"))
            .map(|c| serde_json::from_value(c.clone()))
            .collect();
        return clusters.map_err(MispError::Parse);
    }
    Err(MispError::InvalidResponse(
        "unexpected cluster search format".into(),
    ))
}
