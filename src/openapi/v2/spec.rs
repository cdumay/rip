use std::collections::BTreeMap;

use crate::openapi::v2::documentation::ExternalDoc;
use crate::openapi::v2::info::Info;
use crate::openapi::v2::parameter::Parameter;
use crate::openapi::v2::path::PathItem;
use crate::openapi::v2::response::Response;
use crate::openapi::v2::schema::Schema;
use crate::openapi::v2::security::Security;
use crate::openapi::v2::tag::Tag;
use crate::rest::application::ApplicationGetters;
use crate::rest::scheme::Scheme;
use crate::rest::server::ServerGetters;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    swagger: String,
    info: Info,
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schemes: Option<Vec<Scheme>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    produces: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<Tag>>,
    paths: BTreeMap<String, PathItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    definitions: Option<BTreeMap<String, Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<BTreeMap<String, Parameter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    responses: Option<BTreeMap<String, Response>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_definitions: Option<BTreeMap<String, Security>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security: Option<Vec<BTreeMap<String, Vec<String>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_docs: Option<Vec<ExternalDoc>>,
}


impl<A: ApplicationGetters> From<&A> for Spec {
    fn from(app: &A) -> Spec {
        Spec {
            swagger: "2.0".to_string(),
            info: Info::from(app),
            host: match app.servers().len() > 0 {
                true => Some(app.servers()[0].url().clone().into_string()),
                false => None
            },
            base_path: app.base_url().clone(),
            schemes: app.scheme().clone(),
            consumes: match app.consumes() {
                Some(mimes) => {
                    let mut data = Vec::new();
                    for mime in mimes {
                        data.push(format!("{}", mime));
                    }
                    Some(data)
                }
                None => None
            },
            produces: match app.produces() {
                Some(mimes) => {
                    let mut data = Vec::new();
                    for mime in mimes {
                        data.push(format!("{}", mime));
                    }
                    Some(data)
                }
                None => None
            },
            tags: None,
            paths: BTreeMap::new(),
            definitions: None,
            parameters: None,
            responses: None,
            security_definitions: None,
            security: None,
            external_docs: match app.documentation() {
                Some(doc) => Some(vec![ExternalDoc::from(doc)]),
                None => None
            },
        }
    }
}
