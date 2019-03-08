use std::collections::BTreeMap;

use crate::openapi::v3::component::ObjectOrReference;
use crate::openapi::v3::header::Header;
use crate::openapi::v3::link::Link;
use crate::openapi::v3::media::MediaType;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<BTreeMap<String, ObjectOrReference<Header>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<BTreeMap<String, MediaType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<BTreeMap<String, ObjectOrReference<Link>>>,
}



