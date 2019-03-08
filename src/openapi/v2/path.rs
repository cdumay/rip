use crate::openapi::v2::operation::Operation;
use crate::openapi::v2::pref::ParameterOrRef;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PathItem {
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
    parameters: Option<Vec<ParameterOrRef>>,
}



