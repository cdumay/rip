#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Example {
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}



