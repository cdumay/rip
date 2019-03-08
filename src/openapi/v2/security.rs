use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type")]
pub enum Security {
    #[serde(rename = "apiKey")]
    ApiKey {
        name: String,
        #[serde(rename = "in")]
        location: String,
    },
    #[serde(rename = "oauth2")]
    Oauth2 {
        flow: String,
        #[serde(rename = "authorizationUrl")]
        #[serde(with = "url_serde")]
        authorization_url: url::Url,
        #[serde(rename = "tokenUrl")]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(with = "url_serde")]
        token_url: Option<url::Url>,
        scopes: BTreeMap<String, String>,
    },
    #[serde(rename = "basic")]
    Basic,
}
