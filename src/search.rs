//! High-level search builders for common threat hunting patterns.

use crate::attributes::{AttributeSearchQuery, AttributesClient};
use crate::error::MispError;
use crate::events::{EventSearchQuery, EventsClient};
use crate::models::{Attribute, Event};

pub struct SearchBuilder<'a> {
    events_client: &'a EventsClient,
    attributes_client: &'a AttributesClient,
}

impl<'a> SearchBuilder<'a> {
    pub fn new(events: &'a EventsClient, attributes: &'a AttributesClient) -> Self {
        Self {
            events_client: events,
            attributes_client: attributes,
        }
    }

    pub fn ioc(&self, value: &str) -> IocSearch<'a> {
        IocSearch {
            attributes_client: self.attributes_client,
            value: value.to_string(),
            attr_types: None,
            tags: None,
            not_tags: None,
            from: None,
            to: None,
            to_ids_only: false,
            include_sightings: false,
            include_galaxies: false,
            limit: None,
        }
    }

    pub fn by_tag(&self, tag: &str) -> TagSearch<'a> {
        TagSearch {
            events_client: self.events_client,
            attributes_client: self.attributes_client,
            tags: vec![tag.to_string()],
            not_tags: Vec::new(),
            from: None,
            to: None,
            published_only: false,
            limit: None,
        }
    }

    pub fn recent(&self, duration: &str) -> RecentSearch<'a> {
        RecentSearch {
            events_client: self.events_client,
            attributes_client: self.attributes_client,
            duration: duration.to_string(),
            tags: None,
            attr_types: None,
            to_ids_only: false,
            limit: None,
        }
    }

    pub fn threat_actor(&self, name: &str) -> ThreatActorSearch<'a> {
        ThreatActorSearch {
            events_client: self.events_client,
            actor_name: name.to_string(),
            include_attributes: true,
            limit: None,
        }
    }
}

pub struct IocSearch<'a> {
    attributes_client: &'a AttributesClient,
    value: String,
    attr_types: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    not_tags: Option<Vec<String>>,
    from: Option<String>,
    to: Option<String>,
    to_ids_only: bool,
    include_sightings: bool,
    include_galaxies: bool,
    limit: Option<u32>,
}

impl<'a> IocSearch<'a> {
    pub fn types(mut self, types: Vec<&str>) -> Self {
        self.attr_types = Some(types.into_iter().map(String::from).collect());
        self
    }

    pub fn tags(mut self, tags: Vec<&str>) -> Self {
        self.tags = Some(tags.into_iter().map(String::from).collect());
        self
    }

    pub fn exclude_tags(mut self, tags: Vec<&str>) -> Self {
        self.not_tags = Some(tags.into_iter().map(String::from).collect());
        self
    }

    pub fn from_date(mut self, from: &str) -> Self {
        self.from = Some(from.to_string());
        self
    }

    pub fn to_date(mut self, to: &str) -> Self {
        self.to = Some(to.to_string());
        self
    }

    pub fn to_ids_only(mut self) -> Self {
        self.to_ids_only = true;
        self
    }

    pub fn with_sightings(mut self) -> Self {
        self.include_sightings = true;
        self
    }

    pub fn with_galaxies(mut self) -> Self {
        self.include_galaxies = true;
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub async fn execute(self) -> Result<Vec<Attribute>, MispError> {
        let mut query = AttributeSearchQuery::new().value(&self.value);

        if let Some(tags) = self.tags {
            query = query.tags(tags);
        }
        if let Some(not_tags) = self.not_tags {
            query = query.exclude_tags(not_tags);
        }
        if let Some(from) = self.from {
            query = query.from_date(from);
        }
        if let Some(to) = self.to {
            query = query.to_date(to);
        }
        if self.to_ids_only {
            query = query.to_ids(true);
        }
        if self.include_sightings {
            query = query.include_sightings();
        }
        if self.include_galaxies {
            query = query.include_galaxy();
        }
        if let Some(limit) = self.limit {
            query = query.limit(limit);
        }

        let mut results = self.attributes_client.search(query).await?;

        if let Some(ref types) = self.attr_types {
            results.retain(|a| types.contains(&a.attr_type));
        }

        Ok(results)
    }
}

pub struct TagSearch<'a> {
    events_client: &'a EventsClient,
    attributes_client: &'a AttributesClient,
    tags: Vec<String>,
    not_tags: Vec<String>,
    from: Option<String>,
    to: Option<String>,
    published_only: bool,
    limit: Option<u32>,
}

impl<'a> TagSearch<'a> {
    pub fn and_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    pub fn exclude_tag(mut self, tag: &str) -> Self {
        self.not_tags.push(tag.to_string());
        self
    }

    pub fn from_date(mut self, from: &str) -> Self {
        self.from = Some(from.to_string());
        self
    }

    pub fn to_date(mut self, to: &str) -> Self {
        self.to = Some(to.to_string());
        self
    }

    pub fn published_only(mut self) -> Self {
        self.published_only = true;
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub async fn events(self) -> Result<Vec<Event>, MispError> {
        let mut query = EventSearchQuery::new().tags(self.tags.clone());

        if !self.not_tags.is_empty() {
            query = query.exclude_tags(self.not_tags.clone());
        }
        if let Some(from) = self.from.clone() {
            query = query.from_date(from);
        }
        if let Some(to) = self.to.clone() {
            query = query.to_date(to);
        }
        if self.published_only {
            query = query.published(true);
        }
        if let Some(limit) = self.limit {
            query = query.limit(limit);
        }

        self.events_client.search(query).await
    }

    pub async fn attributes(self) -> Result<Vec<Attribute>, MispError> {
        let mut query = AttributeSearchQuery::new().tags(self.tags.clone());

        if !self.not_tags.is_empty() {
            query = query.exclude_tags(self.not_tags.clone());
        }
        if let Some(from) = self.from.clone() {
            query = query.from_date(from);
        }
        if let Some(to) = self.to.clone() {
            query = query.to_date(to);
        }
        if self.published_only {
            query = query.published(true);
        }
        if let Some(limit) = self.limit {
            query = query.limit(limit);
        }

        self.attributes_client.search(query).await
    }
}

pub struct RecentSearch<'a> {
    events_client: &'a EventsClient,
    attributes_client: &'a AttributesClient,
    duration: String,
    tags: Option<Vec<String>>,
    attr_types: Option<Vec<String>>,
    to_ids_only: bool,
    limit: Option<u32>,
}

impl<'a> RecentSearch<'a> {
    pub fn tags(mut self, tags: Vec<&str>) -> Self {
        self.tags = Some(tags.into_iter().map(String::from).collect());
        self
    }

    pub fn types(mut self, types: Vec<&str>) -> Self {
        self.attr_types = Some(types.into_iter().map(String::from).collect());
        self
    }

    pub fn to_ids_only(mut self) -> Self {
        self.to_ids_only = true;
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub async fn events(self) -> Result<Vec<Event>, MispError> {
        let mut query = EventSearchQuery::new().last(&self.duration);

        if let Some(tags) = self.tags.clone() {
            query = query.tags(tags);
        }
        if let Some(limit) = self.limit {
            query = query.limit(limit);
        }

        self.events_client.search(query).await
    }

    pub async fn attributes(self) -> Result<Vec<Attribute>, MispError> {
        let mut query = AttributeSearchQuery::new().last(&self.duration);

        if let Some(tags) = self.tags.clone() {
            query = query.tags(tags);
        }
        if self.to_ids_only {
            query = query.to_ids(true);
        }
        if let Some(limit) = self.limit {
            query = query.limit(limit);
        }

        let mut results = self.attributes_client.search(query).await?;

        if let Some(ref types) = self.attr_types {
            results.retain(|a| types.contains(&a.attr_type));
        }

        Ok(results)
    }
}

pub struct ThreatActorSearch<'a> {
    events_client: &'a EventsClient,
    actor_name: String,
    include_attributes: bool,
    limit: Option<u32>,
}

impl<'a> ThreatActorSearch<'a> {
    pub fn without_attributes(mut self) -> Self {
        self.include_attributes = false;
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub async fn execute(self) -> Result<Vec<Event>, MispError> {
        let tag = format!("misp-galaxy:threat-actor=\"{}\"", self.actor_name);
        let mut query = EventSearchQuery::new().tags(vec![tag]);

        if self.include_attributes {
            query = query.include_attributes();
        }
        if let Some(limit) = self.limit {
            query = query.limit(limit);
        }

        self.events_client.search(query).await
    }
}
