use crate::openapi::v3::component::ObjectOrReference;
use crate::openapi::v3::operation::Operation;
use crate::openapi::v3::parameter::Parameter;
use crate::openapi::v3::server::Server;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PathItem {
    #[serde(skip_serializing_if = "Option::is_none", rename = "$ref")]
    reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    put: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patch: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trace: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    servers: Option<Vec<Server>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<ObjectOrReference<Parameter>>>,
}



