//!
//! MISP Automation API
//!
//!  ### Getting Started  MISP API allows you to query, create, modify data models, such as [Events](https://www.circl.lu/doc/misp/GLOSSARY.html#misp-event), [Objects](https://www.circl.lu/doc/misp/misp-objects/), [Attributes](https://www.circl.lu/doc/misp/GLOSSARY.html#misp-attribute). This is extremly useful for interconnecting MISP with external tools and feeding other systems with threat intel data.  It also lets you perform administrative tasks such as creating users, organisations, altering MISP settings, and much more.  To get an API key there are several options: * **[UI]** Go to [My Profile -> Auth Keys](/auth_keys/index) section and click on `+ Add authentication key`  * **[UI]** As an admin go to the the [Administration -> List Users -> View](/admin/users/view/[id]) page of the user you want to create an auth key for and on the `Auth keys` section click on `+ Add authentication key`  * **[CLI]** Use the following command: `./app/Console/cake user change_authkey [e-mail/user_id]`  * **API** Provided you already have an admin level API key, you can create an API key for another user using the `[POST]/auth_keys/add/{{user_id}}` endpoint.  > **NOTE:** The authentication key will only be displayed once, so take note of it or store it properly in your application secrets.  #### Accept and Content-Type headers  When performing your request, depending on the type of request, you might need to explicitly specify in what content  type you want to get your results. This is done by setting one of the below `Accept` headers:      Accept: application/json     Accept: application/xml  When submitting data in a `POST`, `PUT` or `DELETE` operation you also need to specify in what content-type you encoded the payload.  This is done by setting one of the below `Content-Type` headers:      Content-Type: application/json     Content-Type: application/xml  Example: ``` curl  --header \"Authorization: YOUR_API_KEY\" \\       --header \"Accept: application/json\" \\       --header \"Content-Type: application/json\" https://<misp url>/  ```  > **NOTE**: By appending .json or .xml the content type can also be set without the need for a header.  #### Automation using PyMISP  [PyMISP](https://github.com/MISP/PyMISP) is a Python library to access MISP platforms via their REST [API](https://www.circl.lu/doc/misp/GLOSSARY.html#api). It allows you to fetch events, add or update events/attributes, add or update samples or search for attributes.  ### FAQ * [Dev FAQ](https://www.circl.lu/doc/misp/dev-faq/) * [GitHub project FAQ](https://github.com/MISP/MISP/wiki/Frequently-Asked-Questions) 
//!
//! The version of the OpenAPI document: 2.4
//! 
//! Generated by: https://openapi-generator.tech
//! 


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`add_object`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddObjectError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_object`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteObjectError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_object_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetObjectByIdError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`rest_search_objects`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RestSearchObjectsError {
    Status403(models::UnauthorizedApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}


pub async fn add_object(configuration: &configuration::Configuration, event_id: &str, object_template_id: &str, add_object_request: Option<models::AddObjectRequest>) -> Result<models::AddObject200Response, Error<AddObjectError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_event_id = event_id;
    let p_object_template_id = object_template_id;
    let p_add_object_request = add_object_request;

    let uri_str = format!("{}/objects/add/{eventId}/{objectTemplateId}", configuration.base_path, eventId=p_event_id.to_string(), objectTemplateId=p_object_template_id.to_string());
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_add_object_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AddObject200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AddObject200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AddObjectError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_object(configuration: &configuration::Configuration, object_id: &str, hard_delete: &str) -> Result<models::DeleteObject200Response, Error<DeleteObjectError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_object_id = object_id;
    let p_hard_delete = hard_delete;

    let uri_str = format!("{}/objects/delete/{objectId}/{hardDelete}", configuration.base_path, objectId=p_object_id.to_string(), hardDelete=crate::apis::urlencode(p_hard_delete));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DeleteObject200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DeleteObject200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteObjectError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_object_by_id(configuration: &configuration::Configuration, object_id: &str) -> Result<models::GetObjectById200Response, Error<GetObjectByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_object_id = object_id;

    let uri_str = format!("{}/objects/view/{objectId}", configuration.base_path, objectId=p_object_id.to_string());
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetObjectById200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetObjectById200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetObjectByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// **This is the recommended endpoint for searching objects.** 
pub async fn rest_search_objects(configuration: &configuration::Configuration, object_rest_search_filter: models::ObjectRestSearchFilter) -> Result<models::RestSearchObjects200Response, Error<RestSearchObjectsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_object_rest_search_filter = object_rest_search_filter;

    let uri_str = format!("{}/objects/restsearch", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_object_rest_search_filter);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::RestSearchObjects200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::RestSearchObjects200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<RestSearchObjectsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

