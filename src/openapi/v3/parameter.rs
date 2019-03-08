use crate::openapi::v3::schema::Schema;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Parameter {
    name: String,
    #[serde(rename = "in")]
    location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<Schema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_items: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    param_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}



