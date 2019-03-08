use std::collections::BTreeMap;

use crate::openapi::v3::component::ObjectOrReference;
use crate::openapi::v3::documentation::ExternalDoc;
use crate::openapi::v3::parameter::Parameter;
use crate::openapi::v3::request::RequestBody;
use crate::openapi::v3::response::Response;
use crate::openapi::v3::server::Server;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "externalDocs")]
    external_docs: Option<ExternalDoc>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "operationId")]
    operation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<ObjectOrReference<Parameter>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestBody")]
    request_body: Option<ObjectOrReference<RequestBody>>,
    responses: BTreeMap<String, Response>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    servers: Option<Vec<Server>>,
}



