use crate::openapi::v2::schema::Schema;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ParameterOrRef {
    #[derive(Default)]
    Parameter {
        name: String,
        #[serde(rename = "in")]
        location: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        required: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        schema: Option<Schema>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "uniqueItems")]
        unique_items: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "type")]
        param_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    Ref {
        #[serde(rename = "$ref")]
        ref_path: String,
    },
}
