//! Factory for creating MISP API clients with shared configuration.

use std::time::Duration;
use tracing::debug;

use crate::attributes::AttributesClient;
use crate::client::MispClient;
use crate::error::MispError;
use crate::events::EventsClient;
use crate::galaxies::GalaxiesClient;
use crate::sightings::SightingsClient;
use crate::warninglists::WarninglistsClient;

#[derive(Clone)]
pub struct MispClientFactory {
    base_url: String,
    api_key: String,
    verify_ssl: bool,
    timeout: Duration,
}

impl MispClientFactory {
    pub fn builder() -> MispClientFactoryBuilder {
        MispClientFactoryBuilder::default()
    }

    fn create_client(&self) -> MispClient {
        MispClient::with_timeout(&self.base_url, &self.api_key, self.verify_ssl, self.timeout)
    }

    pub fn events(&self) -> EventsClient {
        debug!("Creating EventsClient");
        EventsClient::new(self.create_client())
    }

    pub fn attributes(&self) -> AttributesClient {
        debug!("Creating AttributesClient");
        AttributesClient::new(self.create_client())
    }

    pub fn galaxies(&self) -> GalaxiesClient {
        debug!("Creating GalaxiesClient");
        GalaxiesClient::new(self.create_client())
    }

    pub fn sightings(&self) -> SightingsClient {
        debug!("Creating SightingsClient");
        SightingsClient::new(self.create_client())
    }

    pub fn warninglists(&self) -> WarninglistsClient {
        debug!("Creating WarninglistsClient");
        WarninglistsClient::new(self.create_client())
    }

    pub fn all(&self) -> MispClients {
        debug!("Creating all MISP clients");
        MispClients {
            events: self.events(),
            attributes: self.attributes(),
            galaxies: self.galaxies(),
            sightings: self.sightings(),
            warninglists: self.warninglists(),
        }
    }

    pub async fn test_connection(&self) -> Result<ServerInfo, MispError> {
        debug!("Testing MISP connection");
        let client = self.create_client();
        let resp = client.get("/servers/getVersion").await?;

        let version = resp
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let perm_sync = resp
            .get("perm_sync")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let perm_sighting = resp
            .get("perm_sighting")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        Ok(ServerInfo {
            version,
            perm_sync,
            perm_sighting,
        })
    }
}

impl std::fmt::Debug for MispClientFactory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MispClientFactory")
            .field("base_url", &self.base_url)
            .field("verify_ssl", &self.verify_ssl)
            .field("timeout", &self.timeout)
            .finish()
    }
}

#[derive(Default)]
pub struct MispClientFactoryBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
    verify_ssl: Option<bool>,
    timeout: Option<Duration>,
}

impl MispClientFactoryBuilder {
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    pub fn verify_ssl(mut self, verify: bool) -> Self {
        self.verify_ssl = Some(verify);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> MispClientFactory {
        MispClientFactory {
            base_url: self.base_url.expect("base_url is required"),
            api_key: self.api_key.expect("api_key is required"),
            verify_ssl: self.verify_ssl.unwrap_or(true),
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
        }
    }
}

#[derive(Debug)]
pub struct MispClients {
    pub events: EventsClient,
    pub attributes: AttributesClient,
    pub galaxies: GalaxiesClient,
    pub sightings: SightingsClient,
    pub warninglists: WarninglistsClient,
}

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub version: String,
    pub perm_sync: bool,
    pub perm_sighting: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_creates_factory() {
        let factory = MispClientFactory::builder()
            .base_url("https://misp.local")
            .api_key("test-key")
            .verify_ssl(false)
            .build();

        let _events = factory.events();
        let _attributes = factory.attributes();
        let _galaxies = factory.galaxies();
    }

    #[test]
    #[should_panic(expected = "base_url is required")]
    fn builder_requires_base_url() {
        MispClientFactory::builder().api_key("test-key").build();
    }

    #[test]
    #[should_panic(expected = "api_key is required")]
    fn builder_requires_api_key() {
        MispClientFactory::builder()
            .base_url("https://misp.local")
            .build();
    }
}
