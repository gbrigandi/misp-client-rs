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


/// struct for typed errors of method [`add_tag`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddTagError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_tag`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTagError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_tag`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditTagError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_tag_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTagByIdError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_tags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTagsError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_tag`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchTagError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}


pub async fn add_tag(configuration: &configuration::Configuration, tag_no_id: Option<models::TagNoId>) -> Result<models::Tag, Error<AddTagError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_tag_no_id = tag_no_id;

    let uri_str = format!("{}/tags/add", configuration.base_path);
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
    req_builder = req_builder.json(&p_tag_no_id);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Tag`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Tag`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AddTagError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_tag(configuration: &configuration::Configuration, tag_id: &str) -> Result<models::DeleteTag200Response, Error<DeleteTagError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_tag_id = tag_id;

    let uri_str = format!("{}/tags/delete/{tagId}", configuration.base_path, tagId=crate::apis::urlencode(p_tag_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DeleteTag200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DeleteTag200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteTagError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn edit_tag(configuration: &configuration::Configuration, tag_id: &str, tag_no_id: Option<models::TagNoId>) -> Result<models::EditTag200Response, Error<EditTagError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_tag_id = tag_id;
    let p_tag_no_id = tag_no_id;

    let uri_str = format!("{}/tags/edit/{tagId}", configuration.base_path, tagId=crate::apis::urlencode(p_tag_id));
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
    req_builder = req_builder.json(&p_tag_no_id);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::EditTag200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::EditTag200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EditTagError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_tag_by_id(configuration: &configuration::Configuration, tag_id: &str) -> Result<models::Tag, Error<GetTagByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_tag_id = tag_id;

    let uri_str = format!("{}/tags/view/{tagId}", configuration.base_path, tagId=crate::apis::urlencode(p_tag_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Tag`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Tag`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTagByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_tags(configuration: &configuration::Configuration, ) -> Result<models::GetTags200Response, Error<GetTagsError>> {

    let uri_str = format!("{}/tags", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetTags200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetTags200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTagsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn search_tag(configuration: &configuration::Configuration, tag_search_term: &str) -> Result<Vec<models::ExtendedTag>, Error<SearchTagError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_tag_search_term = tag_search_term;

    let uri_str = format!("{}/tags/search/{tagSearchTerm}", configuration.base_path, tagSearchTerm=crate::apis::urlencode(p_tag_search_term));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::ExtendedTag&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::ExtendedTag&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SearchTagError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

