use crate::openapi::v2::schema::Schema;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Response {
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<Schema>,
}



