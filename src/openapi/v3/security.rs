#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type")]
pub enum SecurityScheme {
    #[serde(rename = "apiKey")]
    ApiKey {
        name: String,
        #[serde(rename = "in")]
        location: String,
    },
    #[serde(rename = "http")]
    Http {
        scheme: String,
        #[serde(rename = "bearerFormat")]
        bearer_format: String,
    },
    // FIXME: Implement
    // #[serde(rename = "oauth2")]
    // Oauth2 {
    //     flow: String,
    //     #[serde(rename = "authorizationUrl")]
    //     authorization_url: String,
    //     #[serde(rename = "tokenUrl")]
    //     #[serde(skip_serializing_if = "Option::is_none")]
    //     token_url: Option<String>,
    //     scopes: BTreeMap<String, String>,
    // },
    #[serde(rename = "openIdConnect")]
    OpenIdConnect {
        #[serde(rename = "openIdConnectUrl")]
        open_id_connect_url: String,
    },
}


#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Server {
    #[serde(with = "url_serde")]
    url: url::Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}



