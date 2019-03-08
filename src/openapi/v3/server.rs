use crate::rest::server::ServerGetters;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Server {
    #[serde(with = "url_serde")]
    url: url::Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl<S: ServerGetters> From<&S> for Server {
    fn from(server: &S) -> Server {
        Server {
            url: server.url().clone(),
            description: server.description().clone(),
        }
    }
}