use std::collections::BTreeMap;

use crate::openapi::v3::media::MediaType;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    content: BTreeMap<String, MediaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
}



