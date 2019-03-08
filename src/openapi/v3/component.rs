use std::collections::BTreeMap;

use crate::openapi::v3::example::Example;
use crate::openapi::v3::header::Header;
use crate::openapi::v3::link::Link;
use crate::openapi::v3::parameter::Parameter;
use crate::openapi::v3::request::RequestBody;
use crate::openapi::v3::response::Response;
use crate::openapi::v3::schema::Schema;
use crate::openapi::v3::security::SecurityScheme;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Callback(String);

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ObjectOrReference<T> {
    Object(T),
    Ref {
        #[serde(rename = "$ref")]
        ref_path: String,
    },
}


#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Components {
    #[serde(skip_serializing_if = "Option::is_none")]
    schemas: Option<BTreeMap<String, ObjectOrReference<Schema>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    responses: Option<BTreeMap<String, ObjectOrReference<Response>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<BTreeMap<String, ObjectOrReference<Parameter>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    examples: Option<BTreeMap<String, ObjectOrReference<Example>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_bodies: Option<BTreeMap<String, ObjectOrReference<RequestBody>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<BTreeMap<String, ObjectOrReference<Header>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_schemes: Option<BTreeMap<String, ObjectOrReference<SecurityScheme>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<BTreeMap<String, ObjectOrReference<Link>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    callbacks: Option<BTreeMap<String, ObjectOrReference<Callback>>>,
}



