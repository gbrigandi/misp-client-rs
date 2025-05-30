//!
//! MISP Automation API
//!
//!  ### Getting Started  MISP API allows you to query, create, modify data models, such as [Events](https://www.circl.lu/doc/misp/GLOSSARY.html#misp-event), [Objects](https://www.circl.lu/doc/misp/misp-objects/), [Attributes](https://www.circl.lu/doc/misp/GLOSSARY.html#misp-attribute). This is extremly useful for interconnecting MISP with external tools and feeding other systems with threat intel data.  It also lets you perform administrative tasks such as creating users, organisations, altering MISP settings, and much more.  To get an API key there are several options: * **[UI]** Go to [My Profile -> Auth Keys](/auth_keys/index) section and click on `+ Add authentication key`  * **[UI]** As an admin go to the the [Administration -> List Users -> View](/admin/users/view/[id]) page of the user you want to create an auth key for and on the `Auth keys` section click on `+ Add authentication key`  * **[CLI]** Use the following command: `./app/Console/cake user change_authkey [e-mail/user_id]`  * **API** Provided you already have an admin level API key, you can create an API key for another user using the `[POST]/auth_keys/add/{{user_id}}` endpoint.  > **NOTE:** The authentication key will only be displayed once, so take note of it or store it properly in your application secrets.  #### Accept and Content-Type headers  When performing your request, depending on the type of request, you might need to explicitly specify in what content  type you want to get your results. This is done by setting one of the below `Accept` headers:      Accept: application/json     Accept: application/xml  When submitting data in a `POST`, `PUT` or `DELETE` operation you also need to specify in what content-type you encoded the payload.  This is done by setting one of the below `Content-Type` headers:      Content-Type: application/json     Content-Type: application/xml  Example: ``` curl  --header \"Authorization: YOUR_API_KEY\" \\       --header \"Accept: application/json\" \\       --header \"Content-Type: application/json\" https://<misp url>/  ```  > **NOTE**: By appending .json or .xml the content type can also be set without the need for a header.  #### Automation using PyMISP  [PyMISP](https://github.com/MISP/PyMISP) is a Python library to access MISP platforms via their REST [API](https://www.circl.lu/doc/misp/GLOSSARY.html#api). It allows you to fetch events, add or update events/attributes, add or update samples or search for attributes.  ### FAQ * [Dev FAQ](https://www.circl.lu/doc/misp/dev-faq/) * [GitHub project FAQ](https://github.com/MISP/MISP/wiki/Frequently-Asked-Questions) 
//!
//! The version of the OpenAPI document: 2.4
//! 
//! Generated by: https://openapi-generator.tech
//! 

use crate::models;
use serde::{Deserialize, Serialize};

use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerNoId {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "authkey", skip_serializing_if = "Option::is_none")]
    pub authkey: Option<String>,
    #[serde(rename = "org_id", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(rename = "push", skip_serializing_if = "Option::is_none")]
    pub push: Option<bool>,
    #[serde(rename = "pull", skip_serializing_if = "Option::is_none")]
    pub pull: Option<bool>,
    #[serde(rename = "push_sightings", skip_serializing_if = "Option::is_none")]
    pub push_sightings: Option<bool>,
    #[serde(rename = "push_galaxy_clusters", skip_serializing_if = "Option::is_none")]
    pub push_galaxy_clusters: Option<bool>,
    #[serde(rename = "pull_galaxy_clusters", skip_serializing_if = "Option::is_none")]
    pub pull_galaxy_clusters: Option<bool>,
    #[serde(rename = "lastpulledid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub lastpulledid: Option<Option<String>>,
    #[serde(rename = "lastpushedid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub lastpushedid: Option<Option<String>>,
    #[serde(rename = "organization", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Option<String>>,
    #[serde(rename = "remote_org_id", skip_serializing_if = "Option::is_none")]
    pub remote_org_id: Option<String>,
    #[serde(rename = "publish_without_email", skip_serializing_if = "Option::is_none")]
    pub publish_without_email: Option<bool>,
    #[serde(rename = "unpublish_event", skip_serializing_if = "Option::is_none")]
    pub unpublish_event: Option<bool>,
    #[serde(rename = "self_signed", skip_serializing_if = "Option::is_none")]
    pub self_signed: Option<bool>,
    /// Stringified JSON rules for pulling events from this server.
    #[serde(rename = "pull_rules", skip_serializing_if = "Option::is_none")]
    pub pull_rules: Option<String>,
    /// Stringified JSON rules for pushing events from this server.
    #[serde(rename = "push_rules", skip_serializing_if = "Option::is_none")]
    pub push_rules: Option<String>,
    /// Base64 encoded certificate
    #[serde_as(as = "super::DoubleOption<serde_with::base64::Base64>")]
    #[serde(rename = "cert_file", default, skip_serializing_if = "Option::is_none")]
    pub cert_file: Option<Option<Vec<u8>>>,
    /// Base64 encoded client certificate
    #[serde_as(as = "super::DoubleOption<serde_with::base64::Base64>")]
    #[serde(rename = "client_cert_file", default, skip_serializing_if = "Option::is_none")]
    pub client_cert_file: Option<Option<Vec<u8>>>,
    #[serde(rename = "internal", skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    #[serde(rename = "skip_proxy", skip_serializing_if = "Option::is_none")]
    pub skip_proxy: Option<bool>,
    #[serde(rename = "caching_enabled", skip_serializing_if = "Option::is_none")]
    pub caching_enabled: Option<bool>,
    #[serde(rename = "priority", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub priority: Option<Option<String>>,
    #[serde(rename = "cache_timestamp", skip_serializing_if = "Option::is_none")]
    pub cache_timestamp: Option<bool>,
}

impl ServerNoId {
    pub fn new() -> ServerNoId {
        ServerNoId {
            name: None,
            url: None,
            authkey: None,
            org_id: None,
            push: None,
            pull: None,
            push_sightings: None,
            push_galaxy_clusters: None,
            pull_galaxy_clusters: None,
            lastpulledid: None,
            lastpushedid: None,
            organization: None,
            remote_org_id: None,
            publish_without_email: None,
            unpublish_event: None,
            self_signed: None,
            pull_rules: None,
            push_rules: None,
            cert_file: None,
            client_cert_file: None,
            internal: None,
            skip_proxy: None,
            caching_enabled: None,
            priority: None,
            cache_timestamp: None,
        }
    }
}

