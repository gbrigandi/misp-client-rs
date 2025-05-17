# MISP Client for Rust

A Rust client library for interacting with MISP (Malware Information Sharing Platform) instances via their REST API.

## What is MISP?

[MISP](https://www.misp-project.org/) (Malware Information Sharing Platform) is an open-source threat intelligence platform that facilitates the collection, storage, and sharing of Indicators of Compromise (IoCs) and threat intelligence. It enables organizations to share, store, and correlate information about cyber security incidents, threats, and malware.

Key features of MISP include:
- Sharing of IoCs and threat intelligence between organizations
- Automatic correlation of attributes to identify relationships between events
- Flexible data model with support for various attribute types
- Integration with other security tools and platforms
- Support for taxonomies, galaxies, and other classification systems
- Collaborative analysis of threats

## About This Client

This Rust client library provides a comprehensive API to interact with MISP instances. It was generated using [OpenAPI Generator](https://openapi-generator.tech) from a fine-tuned OpenAPI specification that accurately describes the MISP REST API.

The client supports all major MISP API endpoints, including:
- Events management
- Attributes handling
- Objects manipulation
- Galaxies and taxonomies
- Users and organizations administration
- Sharing groups
- And much more

## Installation

Add the package to your `Cargo.toml`:

```toml
```toml
[dependencies]
misp-client-rs = "0.1.0" # Or the version you intend to publish
```

## Usage Examples

### Example 1: Authenticating and Retrieving Events

```rust
use misp_client_rs::apis::{configuration::Configuration, events_api};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Configure API key authorization
    let mut config = Configuration::new();
    config.base_path = "https://your-misp-instance.com".to_string();
    config.api_key = Some(misp_client_rs::apis::configuration::ApiKey {
        prefix: None,
        key: "YOUR_API_KEY".to_string(),
    });

    // Retrieve all events
    let events = events_api::get_events(&config).await?;
    
    // Print event information
    for event in events {
        println!("Event ID: {}", event.id);
        println!("Event Info: {}", event.info);
        println!("Event Date: {}", event.date);
        println!("Organization: {}", event.org_id);
        println!("Attributes count: {}", event.attribute_count);
        println!("----------------------------");
    }

    Ok(())
}
```

### Example 2: Searching for Specific Attributes

```rust
use misp_client_rs::apis::{configuration::Configuration, attributes_api};
use misp_client_rs::models::AttributeRestSearchFilter; // Removed unused AttributesRestSearchReturnFormat
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Configure API key authorization
    let mut config = Configuration::new();
    config.base_path = "https://your-misp-instance.com".to_string();
    config.api_key = Some(misp_client_rs::apis::configuration::ApiKey {
        prefix: None,
        key: "YOUR_API_KEY".to_string(),
    });

    // Create search filter
    let filter = AttributeRestSearchFilter {
        value: Some("malicious.com".to_string()),
        type_: Some("domain".to_string().into()), // Assuming "domain" is a valid AttributeType variant
        category: None,
        org: None,
        tags: None,
        from: None,
        to: None,
        last: None,
        // event_id: None, // Removed redundant event_id, using eventid as per model comment
        event_info: None,
        threat_level_id: None,
        distribution: None,
        analysis: None,
        org_c: None,
        timestamp: None,
        published: None,
        enforceWarninglist: None,
        to_ids: None,
        deleted: None,
        include_event_uuid: None,
        include_event_tags: None,
        event_timestamp: None,
        sg_reference_only: None,
        eventid: None, // Use this for event ID filtering
        uuid: None,
        publish_timestamp: None,
        sharinggroup: None,
        ..Default::default() // Initialize other fields to None
    };

    // Search for attributes
    let result = attributes_api::rest_search_attributes(
        &config,
        filter,
    ).await?;

    // Process results
    if let Some(attributes_container) = result.response {
        if let Some(attributes) = attributes_container.attribute {
            println!("Found {} attributes", attributes.len());
            for attribute_item in attributes {
                println!("Attribute ID: {}", attribute_item.id.as_deref().unwrap_or("N/A"));
                println!("Value: {}", attribute_item.value.as_deref().unwrap_or("N/A"));
                println!("Type: {}", attribute_item.r#type.as_ref().map_or("N/A".to_string(), |t| t.to_string()));
                println!("Event ID: {}", attribute_item.event_id.as_deref().unwrap_or("N/A"));
                println!("----------------------------");
            }
        } else {
            println!("No attributes found in the response.");
        }
    } else {
        println!("No response field in the result.");
    }

    Ok(())
}
```

### Example 3: Creating an Event and Adding an Attribute

```rust
use misp_client_rs::apis::{configuration::Configuration, events_api, attributes_api};
use misp_client_rs::models::{
    EventNoId, AttributeNoId, AttributeType, AttributeCategory, DistributionLevelId, 
    AnalysisLevelId, ThreatLevelId
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Configure API key authorization
    let mut config = Configuration::new();
    config.base_path = "https://your-misp-instance.com".to_string();
    config.api_key = Some(misp_client_rs::apis::configuration::ApiKey {
        prefix: None,
        key: "YOUR_API_KEY".to_string(),
    });

    // 1. Create a new event
    let new_event_data = EventNoId {
        info: Some("New Event Created via API Client - Ransomware Campaign X".to_string()),
        distribution: Some(DistributionLevelId::Variant0), // Your organization only
        analysis: Some(AnalysisLevelId::Variant0), // Initial
        threat_level_id: Some(ThreatLevelId::Variant1), // High
        date: Some("2024-05-17".to_string()), // Set to the appropriate date
        ..Default::default()
    };

    let created_event_response = events_api::add_event(&config, new_event_data).await?;
    
    let event_id = if let Some(event_details_boxed) = created_event_response.event {
        if let Some(event_id_str) = event_details_boxed.id {
            event_id_str
        } else {
            return Err("Event ID not found in response after creation".into());
        }
    } else {
        return Err("Event details not found in response after creation".into());
    };
    println!("Successfully created event with ID: {}", event_id);

    // 2. Add an attribute to the new event
    let new_attribute_data = AttributeNoId {
        event_id: Some(event_id.clone()), // Link to the newly created event
        r#type: Some(AttributeType::Domain),
        value: Some("example-malicious-domain.com".to_string()),
        category: Some(AttributeCategory::NetworkActivity),
        to_ids: Some(false),
        comment: Some("C2 domain associated with Ransomware Campaign X".to_string()),
        distribution: Some(DistributionLevelId::Variant0),
        ..Default::default()
    };

    let added_attribute_response = attributes_api::add_attribute(&config, &event_id, new_attribute_data).await?;
    
    if let Some(attribute_details_boxed) = added_attribute_response.attribute {
        println!("Successfully added attribute with ID: {} and value: '{}' to event ID: {}",
            attribute_details_boxed.id.as_deref().unwrap_or("N/A"),
            attribute_details_boxed.value.as_deref().unwrap_or("N/A"),
            event_id
        );
    } else {
        println!("Attribute added, but details not found in response.");
    }

    Ok(())
}
```


All URIs are relative to *https://misp.local*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AnalystDataApi* | [**add_analyst_data**](docs/AnalystDataApi.md#add_analyst_data) | **POST** /analystData/add/{analystType}/{analystObjectUUID}/{analystObjectType} | Add analyst data
*AnalystDataApi* | [**delete_analyst_data**](docs/AnalystDataApi.md#delete_analyst_data) | **DELETE** /analystData/delete/{analystType}/{analystDataID} | Delete Analyst data
*AnalystDataApi* | [**edit_analyst_data**](docs/AnalystDataApi.md#edit_analyst_data) | **POST** /analystData/edit/{analystType}/{analystDataID} | Edit analyst data
*AnalystDataApi* | [**get_analyst_data_by_id**](docs/AnalystDataApi.md#get_analyst_data_by_id) | **GET** /analystData/view/{analystType}/{analystDataID} | Get Analyst Data by ID
*AnalystDataApi* | [**index_analyst_data**](docs/AnalystDataApi.md#index_analyst_data) | **GET** /analystData/index/{analystType} | List Analyst data
*AnalystDataApi* | [**index_minimal_analyst_data**](docs/AnalystDataApi.md#index_minimal_analyst_data) | **GET** /analystData/indexMinimal | List minimal Analyst data
*AttributesApi* | [**add_attribute**](docs/AttributesApi.md#add_attribute) | **POST** /attributes/add/{eventId} | Add an attribute
*AttributesApi* | [**delete_attribute**](docs/AttributesApi.md#delete_attribute) | **DELETE** /attributes/delete/{attributeId} | Delete an attribute
*AttributesApi* | [**describe_attribute_types**](docs/AttributesApi.md#describe_attribute_types) | **GET** /attributes/describeTypes | Get a list of the available attribute types
*AttributesApi* | [**edit_attribute**](docs/AttributesApi.md#edit_attribute) | **PUT** /attributes/edit/{attributeId} | Edit an attribute
*AttributesApi* | [**enrich_attribute**](docs/AttributesApi.md#enrich_attribute) | **POST** /attributes/enrich/{attributeId} | Enrich an attribute with the given modules
*AttributesApi* | [**get_attribute_by_id**](docs/AttributesApi.md#get_attribute_by_id) | **GET** /attributes/view/{attributeId} | Get an attribute by ID
*AttributesApi* | [**get_attribute_statistics**](docs/AttributesApi.md#get_attribute_statistics) | **GET** /attributes/attributeStatistics/{context}/{percentage} | Get the count of attributes per category
*AttributesApi* | [**get_attributes**](docs/AttributesApi.md#get_attributes) | **GET** /attributes | Get a list of attributes
*AttributesApi* | [**rest_search_attributes**](docs/AttributesApi.md#rest_search_attributes) | **POST** /attributes/restSearch | [restSearch] Get a filtered and paginated list of attributes
*AttributesApi* | [**restore_attribute**](docs/AttributesApi.md#restore_attribute) | **POST** /attributes/restore/{attributeId} | Restore an attribute
*AttributesApi* | [**tag_attribute**](docs/AttributesApi.md#tag_attribute) | **POST** /attributes/addTag/{attributeId}/{tagId}/local:{local} | Add tag(s) to attribute(s)
*AttributesApi* | [**untag_attribute**](docs/AttributesApi.md#untag_attribute) | **POST** /attributes/removeTag/{attributeId}/{tagId} | Remove a tag from an attribute
*AuthKeysApi* | [**add_auth_key**](docs/AuthKeysApi.md#add_auth_key) | **POST** /auth_keys/add/{userId} | Add auth keys
*AuthKeysApi* | [**delete_auth_key**](docs/AuthKeysApi.md#delete_auth_key) | **DELETE** /auth_keys/delete/{authKeyId} | Delete auth key
*AuthKeysApi* | [**edit_auth_key**](docs/AuthKeysApi.md#edit_auth_key) | **POST** /auth_keys/edit/{authKeyId} | Edit auth key
*AuthKeysApi* | [**get_auth_key_by_id**](docs/AuthKeysApi.md#get_auth_key_by_id) | **GET** /auth_keys/view/{authKeyId} | View auth key
*AuthKeysApi* | [**get_auth_keys**](docs/AuthKeysApi.md#get_auth_keys) | **GET** /auth_keys | Get auth keys
*AuthKeysApi* | [**search_auth_keys**](docs/AuthKeysApi.md#search_auth_keys) | **POST** /auth_keys | Search auth keys
*CollectionsApi* | [**add_collection**](docs/CollectionsApi.md#add_collection) | **POST** /collections/add | Add a collection
*CollectionsApi* | [**delete_collection**](docs/CollectionsApi.md#delete_collection) | **DELETE** /collections/delete/{collectionId} | Delete a collection
*CollectionsApi* | [**edit_collection**](docs/CollectionsApi.md#edit_collection) | **POST** /collections/edit/{collectionId} | Edit a collection
*CollectionsApi* | [**get_collection_by_id**](docs/CollectionsApi.md#get_collection_by_id) | **GET** /collections/view/{collectionId} | View a collection by ID
*CollectionsApi* | [**get_collections**](docs/CollectionsApi.md#get_collections) | **GET** /collections/index/{filter} | Get a list of collections with optional filtering
*EventReportApi* | [**add_event_report**](docs/EventReportApi.md#add_event_report) | **POST** /eventReports/add/{eventId} | Add Event Report
*EventReportApi* | [**delete_event_report**](docs/EventReportApi.md#delete_event_report) | **POST** /eventReports/delete/{eventReportId} | Delete Event Report
*EventReportApi* | [**edit_event_report**](docs/EventReportApi.md#edit_event_report) | **POST** /eventReports/edit/{eventReportId} | Edit Event Report
*EventReportApi* | [**hard_delete_event_report**](docs/EventReportApi.md#hard_delete_event_report) | **POST** /eventReports/delete/{eventReportId}/{hardDelete} | Hard Delete Event Report
*EventReportApi* | [**import_from_url_event_report**](docs/EventReportApi.md#import_from_url_event_report) | **POST** /eventReports/importReportFromUrl/{eventId} | Import Report From URL
*EventReportApi* | [**index_event_report**](docs/EventReportApi.md#index_event_report) | **GET** /eventReports/index | Get event reports
*EventReportApi* | [**restore_event_report**](docs/EventReportApi.md#restore_event_report) | **POST** /eventReports/restore/{eventReportId} | Restore Event Report
*EventReportApi* | [**view_event_report**](docs/EventReportApi.md#view_event_report) | **GET** /eventReports/view/{eventReportId} | Get event report by ID
*EventsApi* | [**add_event**](docs/EventsApi.md#add_event) | **POST** /events/add | Add event
*EventsApi* | [**delete_event**](docs/EventsApi.md#delete_event) | **DELETE** /events/delete/{eventId} | Delete event
*EventsApi* | [**edit_event**](docs/EventsApi.md#edit_event) | **PUT** /events/edit/{eventId} | Edit event
*EventsApi* | [**enrich_event**](docs/EventsApi.md#enrich_event) | **POST** /events/enrichEvent/{eventId} | Enrich an event with the given modules
*EventsApi* | [**get_event_by_id**](docs/EventsApi.md#get_event_by_id) | **GET** /events/view/{eventId} | Get event by ID
*EventsApi* | [**get_events**](docs/EventsApi.md#get_events) | **GET** /events | Get a list of events
*EventsApi* | [**publish_event**](docs/EventsApi.md#publish_event) | **POST** /events/publish/{eventId} | Publish an event
*EventsApi* | [**rest_search_events**](docs/EventsApi.md#rest_search_events) | **POST** /events/restSearch | [restSearch] Get a filtered and paginated list of events
*EventsApi* | [**search_events**](docs/EventsApi.md#search_events) | **POST** /events/index | Search events
*EventsApi* | [**tag_event**](docs/EventsApi.md#tag_event) | **POST** /events/addTag/{eventId}/{tagId}/local:{local} | Add event tag
*EventsApi* | [**unpublish_event**](docs/EventsApi.md#unpublish_event) | **POST** /events/unpublish/{eventId} | Unpublish an event
*EventsApi* | [**untag_event**](docs/EventsApi.md#untag_event) | **POST** /events/removeTag/{eventId}/{tagId} | Remove event tag
*FeedsApi* | [**add_feed**](docs/FeedsApi.md#add_feed) | **POST** /feeds/add | Add a feed
*FeedsApi* | [**cache_feeds**](docs/FeedsApi.md#cache_feeds) | **POST** /feeds/cacheFeeds/{cacheFeedsScope} | Cache feeds
*FeedsApi* | [**disable_feed**](docs/FeedsApi.md#disable_feed) | **POST** /feeds/disable/{feedId} | Disable feed
*FeedsApi* | [**edit_feed**](docs/FeedsApi.md#edit_feed) | **PUT** /feeds/edit/{feedId} | Edit a feed
*FeedsApi* | [**enable_feed**](docs/FeedsApi.md#enable_feed) | **POST** /feeds/enable/{feedId} | Enable feed
*FeedsApi* | [**fetch_from_all_feeds**](docs/FeedsApi.md#fetch_from_all_feeds) | **POST** /feeds/fetchFromAllFeeds | Fetch from all feeds
*FeedsApi* | [**fetch_from_feed**](docs/FeedsApi.md#fetch_from_feed) | **POST** /feeds/fetchFromFeed/{feedId} | Fetch from feed by ID
*FeedsApi* | [**get_feed_by_id**](docs/FeedsApi.md#get_feed_by_id) | **GET** /feeds/view/{feedId} | Get a feed by ID
*FeedsApi* | [**get_feeds**](docs/FeedsApi.md#get_feeds) | **GET** /feeds | Get a list of feeds
*GalaxiesApi* | [**attach_galaxy_cluster**](docs/GalaxiesApi.md#attach_galaxy_cluster) | **POST** /galaxies/attachCluster/{attachTargetId}/{attachTargetType}/local:{local} | Attach the galaxy cluster tag a given entity
*GalaxiesApi* | [**delete_galaxy**](docs/GalaxiesApi.md#delete_galaxy) | **DELETE** /galaxies/delete/{galaxyId} | Delete a galaxy
*GalaxiesApi* | [**export_galaxy_clusters**](docs/GalaxiesApi.md#export_galaxy_clusters) | **POST** /galaxies/export/{galaxyId} | Export galaxy clusters
*GalaxiesApi* | [**get_galaxies**](docs/GalaxiesApi.md#get_galaxies) | **GET** /galaxies | Get galaxies
*GalaxiesApi* | [**get_galaxy_by_id**](docs/GalaxiesApi.md#get_galaxy_by_id) | **GET** /galaxies/view/{galaxyId} | Get galaxy by ID
*GalaxiesApi* | [**import_galaxy_cluster**](docs/GalaxiesApi.md#import_galaxy_cluster) | **POST** /galaxies/import | Import a galaxy cluster
*GalaxiesApi* | [**search_galaxies**](docs/GalaxiesApi.md#search_galaxies) | **POST** /galaxies | Search galaxies
*GalaxiesApi* | [**update_galaxies**](docs/GalaxiesApi.md#update_galaxies) | **POST** /galaxies/update | Force update the galaxies with the galaxy json definitions
*GalaxyClustersApi* | [**add_galaxy_cluster**](docs/GalaxyClustersApi.md#add_galaxy_cluster) | **POST** /galaxy_clusters/add/{galaxyId} | Add galaxy cluster
*GalaxyClustersApi* | [**delete_galaxy_cluster**](docs/GalaxyClustersApi.md#delete_galaxy_cluster) | **POST** /galaxy_clusters/delete/{galaxyClusterId} | Delete galaxy cluster
*GalaxyClustersApi* | [**edit_galaxy_cluster**](docs/GalaxyClustersApi.md#edit_galaxy_cluster) | **PUT** /galaxy_clusters/edit/{galaxyClusterId} | Edit galaxy cluster
*GalaxyClustersApi* | [**get_galaxy_cluster_by_id**](docs/GalaxyClustersApi.md#get_galaxy_cluster_by_id) | **GET** /galaxy_clusters/view/{galaxyClusterId} | Get galaxy cluster by ID
*GalaxyClustersApi* | [**get_galaxy_clusters**](docs/GalaxyClustersApi.md#get_galaxy_clusters) | **GET** /galaxy_clusters/index/{galaxyId} | Get galaxy clusters
*GalaxyClustersApi* | [**publish_galaxy_cluster**](docs/GalaxyClustersApi.md#publish_galaxy_cluster) | **POST** /galaxy_clusters/publish/{galaxyClusterId} | Publish galaxy cluster
*GalaxyClustersApi* | [**restore_galaxy_cluster**](docs/GalaxyClustersApi.md#restore_galaxy_cluster) | **POST** /galaxy_clusters/restore/{galaxyClusterId} | Restore galaxy cluster
*GalaxyClustersApi* | [**search_galaxy_clusters**](docs/GalaxyClustersApi.md#search_galaxy_clusters) | **POST** /galaxy_clusters/index/{galaxyId} | Search galaxy clusters
*GalaxyClustersApi* | [**unpublish_galaxy_cluster**](docs/GalaxyClustersApi.md#unpublish_galaxy_cluster) | **POST** /galaxy_clusters/unpublish/{galaxyClusterId} | Unpublish galaxy cluster
*LogsApi* | [**get_logs**](docs/LogsApi.md#get_logs) | **POST** /admin/logs | Get instance logs
*NoticelistsApi* | [**get_noticelist_by_id**](docs/NoticelistsApi.md#get_noticelist_by_id) | **GET** /noticelists/view/{noticelistId} | Get a noticelist by ID
*NoticelistsApi* | [**get_noticelists**](docs/NoticelistsApi.md#get_noticelists) | **GET** /noticelists | Get a list of noticelists
*NoticelistsApi* | [**toggle_enable_noticelist**](docs/NoticelistsApi.md#toggle_enable_noticelist) | **POST** /noticelists/toggleEnable/{noticelistId} | Enable/disable noticelist
*NoticelistsApi* | [**update_noticelists**](docs/NoticelistsApi.md#update_noticelists) | **POST** /noticelists/update | Update noticelists
*ObjectsApi* | [**add_object**](docs/ObjectsApi.md#add_object) | **POST** /objects/add/{eventId}/{objectTemplateId} | Add an object to an event
*ObjectsApi* | [**delete_object**](docs/ObjectsApi.md#delete_object) | **DELETE** /objects/delete/{objectId}/{hardDelete} | Delete object
*ObjectsApi* | [**get_object_by_id**](docs/ObjectsApi.md#get_object_by_id) | **GET** /objects/view/{objectId} | Get object by ID
*ObjectsApi* | [**rest_search_objects**](docs/ObjectsApi.md#rest_search_objects) | **POST** /objects/restsearch | [restSearch] Get a filtered and paginated list of objects
*OrganisationsApi* | [**add_organisation**](docs/OrganisationsApi.md#add_organisation) | **POST** /admin/organisations/add | Add organisation
*OrganisationsApi* | [**delete_organisation**](docs/OrganisationsApi.md#delete_organisation) | **DELETE** /admin/organisations/delete/{organisationId} | Delete organisation
*OrganisationsApi* | [**edit_organisation**](docs/OrganisationsApi.md#edit_organisation) | **PUT** /admin/organisations/edit/{organisationId} | Edit organisation
*OrganisationsApi* | [**get_organisation_by_id**](docs/OrganisationsApi.md#get_organisation_by_id) | **GET** /organisations/view/{organisationId} | Get organisation by ID
*OrganisationsApi* | [**get_organisations**](docs/OrganisationsApi.md#get_organisations) | **GET** /organisations | Get organisations
*ServersApi* | [**add_server**](docs/ServersApi.md#add_server) | **POST** /servers/add | Add server
*ServersApi* | [**cache_server**](docs/ServersApi.md#cache_server) | **POST** /servers/cache | Cache server
*ServersApi* | [**create_sync**](docs/ServersApi.md#create_sync) | **POST** /servers/createSync | Create sync
*ServersApi* | [**delete_server**](docs/ServersApi.md#delete_server) | **POST** /servers/delete/{serverId} | Delete server
*ServersApi* | [**edit_server**](docs/ServersApi.md#edit_server) | **PUT** /servers/edit/{serverId} | Edit server
*ServersApi* | [**edit_server_setting**](docs/ServersApi.md#edit_server_setting) | **POST** /servers/serverSettingsEdit/{settingName} | Edit server setting
*ServersApi* | [**get_py_misp_version**](docs/ServersApi.md#get_py_misp_version) | **GET** /servers/getPyMISPVersion | Get current instance PyMISP version
*ServersApi* | [**get_server_setting**](docs/ServersApi.md#get_server_setting) | **GET** /servers/getSetting/{settingName} | Get server setting by name
*ServersApi* | [**get_server_settings**](docs/ServersApi.md#get_server_settings) | **GET** /servers/serverSettings | Get current instance settings and diagnostics
*ServersApi* | [**get_server_uuid**](docs/ServersApi.md#get_server_uuid) | **GET** /servers/getInstanceUUID | Get instance UUID
*ServersApi* | [**get_server_version**](docs/ServersApi.md#get_server_version) | **GET** /servers/getVersion | Get current instance version
*ServersApi* | [**get_servers**](docs/ServersApi.md#get_servers) | **GET** /servers | Get servers
*ServersApi* | [**get_workers**](docs/ServersApi.md#get_workers) | **GET** /servers/getWorkers | Get workers
*ServersApi* | [**import_server**](docs/ServersApi.md#import_server) | **POST** /servers/import | Import server
*ServersApi* | [**kill_all_workers**](docs/ServersApi.md#kill_all_workers) | **POST** /servers/killAllWorkers | Kill all workers
*ServersApi* | [**pull_server**](docs/ServersApi.md#pull_server) | **GET** /servers/pull/{serverId}/{pullTechnique} | Pull server
*ServersApi* | [**push_server**](docs/ServersApi.md#push_server) | **GET** /servers/push/{serverId}/{pushTechnique} | Push server
*ServersApi* | [**restart_dead_workers**](docs/ServersApi.md#restart_dead_workers) | **POST** /servers/restartDeadWorkers | Restart dead workers
*ServersApi* | [**restart_workers**](docs/ServersApi.md#restart_workers) | **POST** /servers/restartWorkers | Restart workers
*ServersApi* | [**start_worker**](docs/ServersApi.md#start_worker) | **POST** /servers/startWorker/{workerType} | Start worker
*ServersApi* | [**stop_worker**](docs/ServersApi.md#stop_worker) | **POST** /servers/stopWorker/{workerPid} | Stop worker
*ServersApi* | [**update_server**](docs/ServersApi.md#update_server) | **POST** /servers/update | Update server
*SharingGroupsApi* | [**add_organisation_to_sharing_group**](docs/SharingGroupsApi.md#add_organisation_to_sharing_group) | **POST** /sharing_groups/addOrg/{sharingGroupId}/{organisationId} | Add an organisation to a sharing group
*SharingGroupsApi* | [**add_server_to_sharing_group**](docs/SharingGroupsApi.md#add_server_to_sharing_group) | **POST** /sharing_groups/addServer/{sharingGroupId}/{serverId} | Add a server to a sharing group
*SharingGroupsApi* | [**add_sharing_group**](docs/SharingGroupsApi.md#add_sharing_group) | **POST** /sharing_groups/add | Add a sharing group
*SharingGroupsApi* | [**delete_sharing_group**](docs/SharingGroupsApi.md#delete_sharing_group) | **DELETE** /sharing_groups/delete/{sharingGroupId} | Delete a sharing group
*SharingGroupsApi* | [**edit_sharing_group**](docs/SharingGroupsApi.md#edit_sharing_group) | **POST** /sharing_groups/edit/{sharingGroupId} | Edit a sharing group
*SharingGroupsApi* | [**get_sharing_group**](docs/SharingGroupsApi.md#get_sharing_group) | **GET** /sharing_groups | Get a list of sharing groups
*SharingGroupsApi* | [**get_sharing_group_by_id**](docs/SharingGroupsApi.md#get_sharing_group_by_id) | **GET** /sharing_groups/view/{sharingGroupId} | Get a sharing group by ID
*SharingGroupsApi* | [**remove_organisation_from_sharing_group**](docs/SharingGroupsApi.md#remove_organisation_from_sharing_group) | **POST** /sharing_groups/removeOrg/{sharingGroupId}/{organisationId} | Remove an organisation from a sharing group
*SharingGroupsApi* | [**remove_server_from_sharing_group**](docs/SharingGroupsApi.md#remove_server_from_sharing_group) | **POST** /sharing_groups/removeServer/{sharingGroupServerId}/{serverId} | Remove a server from a sharing group
*SightingsApi* | [**add_sighting**](docs/SightingsApi.md#add_sighting) | **POST** /sightings/add/{attributeId} | Add sighting of an attribute
*SightingsApi* | [**add_sighting_by_value**](docs/SightingsApi.md#add_sighting_by_value) | **POST** /sightings/add | Add sightings of a list of values
*SightingsApi* | [**delete_sighting**](docs/SightingsApi.md#delete_sighting) | **POST** /sightings/delete/{sightingId} | Delete sighting
*SightingsApi* | [**get_sightings_by_event_id**](docs/SightingsApi.md#get_sightings_by_event_id) | **GET** /sightings/index/{eventId} | Get sightings by event ID
*TagsApi* | [**add_tag**](docs/TagsApi.md#add_tag) | **POST** /tags/add | Add tag
*TagsApi* | [**delete_tag**](docs/TagsApi.md#delete_tag) | **POST** /tags/delete/{tagId} | Delete tag
*TagsApi* | [**edit_tag**](docs/TagsApi.md#edit_tag) | **POST** /tags/edit/{tagId} | Edit tag
*TagsApi* | [**get_tag_by_id**](docs/TagsApi.md#get_tag_by_id) | **GET** /tags/view/{tagId} | Get tag by ID
*TagsApi* | [**get_tags**](docs/TagsApi.md#get_tags) | **GET** /tags | Get tags
*TagsApi* | [**search_tag**](docs/TagsApi.md#search_tag) | **GET** /tags/search/{tagSearchTerm} | Search tag
*TaxonomiesApi* | [**disable_taxonomy**](docs/TaxonomiesApi.md#disable_taxonomy) | **POST** /taxonomies/disable/{taxonomyId} | Disable taxonomy
*TaxonomiesApi* | [**enable_taxonomy**](docs/TaxonomiesApi.md#enable_taxonomy) | **POST** /taxonomies/enable/{taxonomyId} | Enable taxonomy
*TaxonomiesApi* | [**export_taxonomy**](docs/TaxonomiesApi.md#export_taxonomy) | **GET** /taxonomies/export/{taxonomyId} | Export taxonomy.
*TaxonomiesApi* | [**get_taxonomies**](docs/TaxonomiesApi.md#get_taxonomies) | **GET** /taxonomies | Get taxonomies
*TaxonomiesApi* | [**get_taxonomy_by_id**](docs/TaxonomiesApi.md#get_taxonomy_by_id) | **GET** /taxonomies/view/{taxonomyId} | Get a taxonomy by ID
*TaxonomiesApi* | [**get_taxonomy_tags**](docs/TaxonomiesApi.md#get_taxonomy_tags) | **GET** /taxonomies/taxonomy_tags/{taxonomyId} | Get a taxonomy extended with tags used in events and attributes.
*TaxonomiesApi* | [**update_taxonomies**](docs/TaxonomiesApi.md#update_taxonomies) | **POST** /taxonomies/update | Update taxonomies
*UserSettingsApi* | [**delete_user_setting_by_id**](docs/UserSettingsApi.md#delete_user_setting_by_id) | **DELETE** /user_settings/delete/{userSettingId} | Delete user setting by id
*UserSettingsApi* | [**get_user_setting_by_id**](docs/UserSettingsApi.md#get_user_setting_by_id) | **GET** /user_settings/view/{userSettingId} | Get user setting by id
*UserSettingsApi* | [**get_user_setting_by_name**](docs/UserSettingsApi.md#get_user_setting_by_name) | **GET** /user_settings/getSetting/{userId}/{userSettingName} | Get user setting by id
*UserSettingsApi* | [**get_user_settings**](docs/UserSettingsApi.md#get_user_settings) | **GET** /user_settings | Get user settings
*UserSettingsApi* | [**search_user_settings**](docs/UserSettingsApi.md#search_user_settings) | **POST** /user_settings | Search user settings
*UserSettingsApi* | [**set_user_setting**](docs/UserSettingsApi.md#set_user_setting) | **POST** /user_settings/setSetting/{userId}/{userSettingName} | Set user setting
*UsersApi* | [**add_user**](docs/UsersApi.md#add_user) | **POST** /admin/users/add | Add user
*UsersApi* | [**delete_user**](docs/UsersApi.md#delete_user) | **DELETE** /admin/users/delete/{userId} | Delete user
*UsersApi* | [**delete_user_totp**](docs/UsersApi.md#delete_user_totp) | **DELETE** /users/totp_delete/{userId} | Delete user TOTP
*UsersApi* | [**edit_user**](docs/UsersApi.md#edit_user) | **PUT** /admin/users/edit/{userId} | Edit user
*UsersApi* | [**get_user_by_id**](docs/UsersApi.md#get_user_by_id) | **GET** /admin/users/view/{userId} | Get user by ID
*UsersApi* | [**get_users**](docs/UsersApi.md#get_users) | **GET** /admin/users | Get users
*UsersApi* | [**reset_user_password**](docs/UsersApi.md#reset_user_password) | **POST** /users/initiatePasswordReset/{userId}/{firstTimeReset} | Reset user password
*WarninglistsApi* | [**check_value_warninglists_matches**](docs/WarninglistsApi.md#check_value_warninglists_matches) | **POST** /warninglists/checkValue | Check if a list of values matches any warninglists
*WarninglistsApi* | [**get_warninglist_by_id**](docs/WarninglistsApi.md#get_warninglist_by_id) | **GET** /warninglists/view/{warninglistId} | Get warninglist by ID
*WarninglistsApi* | [**get_warninglists**](docs/WarninglistsApi.md#get_warninglists) | **GET** /warninglists | Get a list of warninglists
*WarninglistsApi* | [**search_warninglists**](docs/WarninglistsApi.md#search_warninglists) | **POST** /warninglists | Search warninglists
*WarninglistsApi* | [**toggle_enable_warninglist**](docs/WarninglistsApi.md#toggle_enable_warninglist) | **POST** /warninglists/toggleEnable | Enable/disable warninglists
*WarninglistsApi* | [**update_warninglists**](docs/WarninglistsApi.md#update_warninglists) | **POST** /warninglists/update | Update warninglists

## Documentation For Models

See the [docs](docs/) directory for detailed documentation of all models.

## License

This project is licensed under the terms specified in the Cargo.toml file.

## Author

This client was generated using OpenAPI Generator from a fine-tuned OpenAPI specification of the MISP API.
