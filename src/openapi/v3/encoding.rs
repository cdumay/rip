use std::collections::BTreeMap;

use crate::openapi::v3::component::ObjectOrReference;
use crate::openapi::v3::header::Header;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Encoding {
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentType")]
    content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<BTreeMap<String, ObjectOrReference<Header>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowReserved")]
    allow_reserved: Option<bool>,
}



