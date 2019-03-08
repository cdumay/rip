use crate::openapi::v3::server::Server;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Link {
    Ref {
        #[serde(rename = "operationRef")]
        operation_ref: String,
        // parameters: BTreeMap<String, Any | {expression}>,
        // #[serde(rename = "requestBody")]
        // request_body: Any | {expression}
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        server: Option<Server>,
    },
    Id {
        #[serde(rename = "operationId")]
        operation_id: String,
        // parameters: BTreeMap<String, Any | {expression}>,
        // #[serde(rename = "requestBody")]
        // request_body: Any | {expression}
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        server: Option<Server>,
    },
}
