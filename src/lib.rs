//! Rust client library for the MISP threat intelligence platform API.

pub mod attributes;
pub mod client;
pub mod client_factory;
pub mod error;
pub mod events;
pub mod galaxies;
pub mod models;
pub mod search;
pub mod sightings;
pub mod warninglists;

pub use attributes::{AttributeSearchQuery, AttributesClient, AttributeTypes};
pub use client::MispClient;
pub use client_factory::{MispClientFactory, MispClientFactoryBuilder, MispClients, ServerInfo};
pub use error::MispError;
pub use events::{EventIndexParams, EventSearchQuery, EventsClient};
pub use galaxies::{ClusterSearchQuery, GalaxiesClient};
pub use models::{
    AnalysisLevel, Attribute, Distribution, Event, Galaxy, GalaxyCluster, GalaxyElement,
    MispObject, ObjectReference, Organisation, Sighting, Tag, ThreatLevel, Warninglist,
    WarninglistCheckResult, WarninglistMatch,
};
pub use search::SearchBuilder;
pub use sightings::{SightingCount, SightingEntry, SightingSearchQuery, SightingType, SightingsClient};
pub use warninglists::WarninglistsClient;

pub type Result<T> = std::result::Result<T, MispError>;
