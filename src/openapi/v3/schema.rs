use std::collections::BTreeMap;

use crate::openapi::v3::component::ObjectOrReference;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Schema {
    #[serde(skip_serializing_if = "Option::is_none", rename = "$ref")]
    ref_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    schema_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "enum")]
    enum_values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Box<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<BTreeMap<String, Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_properties: Option<ObjectOrReference<Box<Schema>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    example: Option<serde_json::value::Value>,
}



