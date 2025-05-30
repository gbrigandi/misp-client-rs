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


/// struct for typed errors of method [`add_feed`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddFeedError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cache_feeds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CacheFeedsError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`disable_feed`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DisableFeedError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_feed`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditFeedError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`enable_feed`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnableFeedError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_from_all_feeds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchFromAllFeedsError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_from_feed`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchFromFeedError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_feed_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFeedByIdError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_feeds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFeedsError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}


pub async fn add_feed(configuration: &configuration::Configuration, add_feed_request: Option<models::AddFeedRequest>) -> Result<models::GetFeeds200ResponseInner, Error<AddFeedError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_add_feed_request = add_feed_request;

    let uri_str = format!("{}/feeds/add", configuration.base_path);
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
    req_builder = req_builder.json(&p_add_feed_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetFeeds200ResponseInner`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetFeeds200ResponseInner`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AddFeedError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn cache_feeds(configuration: &configuration::Configuration, cache_feeds_scope: &str) -> Result<models::CacheFeeds200Response, Error<CacheFeedsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_cache_feeds_scope = cache_feeds_scope;

    let uri_str = format!("{}/feeds/cacheFeeds/{cacheFeedsScope}", configuration.base_path, cacheFeedsScope=crate::apis::urlencode(p_cache_feeds_scope));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CacheFeeds200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CacheFeeds200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CacheFeedsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn disable_feed(configuration: &configuration::Configuration, feed_id: &str) -> Result<models::DisableFeed200Response, Error<DisableFeedError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_feed_id = feed_id;

    let uri_str = format!("{}/feeds/disable/{feedId}", configuration.base_path, feedId=p_feed_id.to_string());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DisableFeed200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DisableFeed200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DisableFeedError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn edit_feed(configuration: &configuration::Configuration, feed_id: &str, edit_feed_request: Option<models::EditFeedRequest>) -> Result<models::GetFeeds200ResponseInner, Error<EditFeedError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_feed_id = feed_id;
    let p_edit_feed_request = edit_feed_request;

    let uri_str = format!("{}/feeds/edit/{feedId}", configuration.base_path, feedId=p_feed_id.to_string());
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

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
    req_builder = req_builder.json(&p_edit_feed_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetFeeds200ResponseInner`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetFeeds200ResponseInner`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EditFeedError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn enable_feed(configuration: &configuration::Configuration, feed_id: &str) -> Result<models::EnableFeed200Response, Error<EnableFeedError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_feed_id = feed_id;

    let uri_str = format!("{}/feeds/enable/{feedId}", configuration.base_path, feedId=p_feed_id.to_string());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::EnableFeed200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::EnableFeed200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EnableFeedError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn fetch_from_all_feeds(configuration: &configuration::Configuration, ) -> Result<models::FetchFromFeed200Response, Error<FetchFromAllFeedsError>> {

    let uri_str = format!("{}/feeds/fetchFromAllFeeds", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::FetchFromFeed200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::FetchFromFeed200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<FetchFromAllFeedsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn fetch_from_feed(configuration: &configuration::Configuration, feed_id: &str) -> Result<models::FetchFromFeed200Response, Error<FetchFromFeedError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_feed_id = feed_id;

    let uri_str = format!("{}/feeds/fetchFromFeed/{feedId}", configuration.base_path, feedId=p_feed_id.to_string());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::FetchFromFeed200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::FetchFromFeed200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<FetchFromFeedError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_feed_by_id(configuration: &configuration::Configuration, feed_id: &str) -> Result<models::GetFeeds200ResponseInner, Error<GetFeedByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_feed_id = feed_id;

    let uri_str = format!("{}/feeds/view/{feedId}", configuration.base_path, feedId=p_feed_id.to_string());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetFeeds200ResponseInner`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetFeeds200ResponseInner`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetFeedByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_feeds(configuration: &configuration::Configuration, ) -> Result<Vec<models::GetFeeds200ResponseInner>, Error<GetFeedsError>> {

    let uri_str = format!("{}/feeds", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::GetFeeds200ResponseInner&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::GetFeeds200ResponseInner&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetFeedsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

