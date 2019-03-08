use std::collections::BTreeMap;

use crate::openapi::v2::pref::ParameterOrRef;
use crate::openapi::v2::response::Response;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    produces: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schemes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_id: Option<String>,
    responses: BTreeMap<String, Response>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<ParameterOrRef>>,
}



