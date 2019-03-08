use std::collections::BTreeMap;

use crate::openapi::v3::component::Components;
use crate::openapi::v3::documentation::ExternalDoc;
use crate::openapi::v3::info::Info;
use crate::openapi::v3::path::PathItem;
use crate::openapi::v3::server::Server;
use crate::openapi::v3::tag::Tag;
use crate::rest::application::ApplicationGetters;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Spec {
    openapi: String,
    info: Info,
    #[serde(skip_serializing_if = "Option::is_none")]
    servers: Option<Vec<Server>>,
    paths: BTreeMap<String, PathItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<Components>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<Tag>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "externalDocs")]
    external_docs: Option<Vec<ExternalDoc>>,
}


impl<A: ApplicationGetters> From<&A> for Spec {
    fn from(app: &A) -> Spec {
        Spec {
            openapi: "3.0".to_string(),
            info: Info::from(app),
            servers: match app.servers().len() > 0 {
                true => Some({
                    let mut servers = Vec::new();
                    for server in app.servers() {
                        servers.push(Server::from(server));
                    }
                    servers
                }),
                false => None
            },
            components: None,
            tags: None,
            paths: BTreeMap::new(),
            external_docs: match app.documentation() {
                Some(doc) => Some(vec![ExternalDoc::from(doc)]),
                None => None
            },
        }
    }
}
