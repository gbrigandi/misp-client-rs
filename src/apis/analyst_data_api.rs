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


/// struct for typed errors of method [`add_analyst_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddAnalystDataError {
    Status403(models::UnauthorizedApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_analyst_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAnalystDataError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_analyst_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditAnalystDataError {
    Status403(models::UnauthorizedApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_analyst_data_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAnalystDataByIdError {
    Status403(models::UnauthorizedApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`index_analyst_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndexAnalystDataError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`index_minimal_analyst_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndexMinimalAnalystDataError {
    Status403(models::UnauthorizedApiError),
    Status404(models::NotFoundApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}


pub async fn add_analyst_data(configuration: &configuration::Configuration, analyst_type: models::AnalystDataType, analyst_object_uuid: &str, analyst_object_type: models::AnalystObjectType, add_analyst_data_request: models::AddAnalystDataRequest) -> Result<models::AddAnalystDataRequest, Error<AddAnalystDataError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_analyst_type = analyst_type;
    let p_analyst_object_uuid = analyst_object_uuid;
    let p_analyst_object_type = analyst_object_type;
    let p_add_analyst_data_request = add_analyst_data_request;

    let uri_str = format!("{}/analystData/add/{analystType}/{analystObjectUUID}/{analystObjectType}", configuration.base_path, analystType=p_analyst_type.to_string(), analystObjectUUID=crate::apis::urlencode(p_analyst_object_uuid), analystObjectType=p_analyst_object_type.to_string());
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
    req_builder = req_builder.json(&p_add_analyst_data_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AddAnalystDataRequest`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AddAnalystDataRequest`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AddAnalystDataError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_analyst_data(configuration: &configuration::Configuration, analyst_type: models::AnalystDataType, analyst_data_id: &str) -> Result<models::DeleteAnalystData200Response, Error<DeleteAnalystDataError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_analyst_type = analyst_type;
    let p_analyst_data_id = analyst_data_id;

    let uri_str = format!("{}/analystData/delete/{analystType}/{analystDataID}", configuration.base_path, analystType=p_analyst_type.to_string(), analystDataID=p_analyst_data_id.to_string());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DeleteAnalystData200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DeleteAnalystData200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteAnalystDataError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn edit_analyst_data(configuration: &configuration::Configuration, analyst_type: models::AnalystDataType, analyst_data_id: &str, add_analyst_data_request: models::AddAnalystDataRequest) -> Result<models::AddAnalystDataRequest, Error<EditAnalystDataError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_analyst_type = analyst_type;
    let p_analyst_data_id = analyst_data_id;
    let p_add_analyst_data_request = add_analyst_data_request;

    let uri_str = format!("{}/analystData/edit/{analystType}/{analystDataID}", configuration.base_path, analystType=p_analyst_type.to_string(), analystDataID=p_analyst_data_id.to_string());
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
    req_builder = req_builder.json(&p_add_analyst_data_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AddAnalystDataRequest`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AddAnalystDataRequest`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EditAnalystDataError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_analyst_data_by_id(configuration: &configuration::Configuration, analyst_type: models::AnalystDataType, analyst_data_id: &str) -> Result<models::AddAnalystDataRequest, Error<GetAnalystDataByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_analyst_type = analyst_type;
    let p_analyst_data_id = analyst_data_id;

    let uri_str = format!("{}/analystData/view/{analystType}/{analystDataID}", configuration.base_path, analystType=p_analyst_type.to_string(), analystDataID=p_analyst_data_id.to_string());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AddAnalystDataRequest`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AddAnalystDataRequest`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAnalystDataByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn index_analyst_data(configuration: &configuration::Configuration, analyst_type: models::AnalystDataType) -> Result<Vec<models::AddAnalystDataRequest>, Error<IndexAnalystDataError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_analyst_type = analyst_type;

    let uri_str = format!("{}/analystData/index/{analystType}", configuration.base_path, analystType=p_analyst_type.to_string());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::AddAnalystDataRequest&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::AddAnalystDataRequest&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<IndexAnalystDataError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn index_minimal_analyst_data(configuration: &configuration::Configuration, index_minimal_analyst_data_request: Option<models::IndexMinimalAnalystDataRequest>) -> Result<models::IndexMinimalAnalystData200Response, Error<IndexMinimalAnalystDataError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_index_minimal_analyst_data_request = index_minimal_analyst_data_request;

    let uri_str = format!("{}/analystData/indexMinimal", configuration.base_path);
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
    req_builder = req_builder.json(&p_index_minimal_analyst_data_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::IndexMinimalAnalystData200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::IndexMinimalAnalystData200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<IndexMinimalAnalystDataError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

