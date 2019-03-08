use crate::rest::documentation::DocumentationGetters;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalDoc {
    #[serde(with = "url_serde")]
    url: url::Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}


impl<D: DocumentationGetters> From<&D> for ExternalDoc {
    fn from(doc: &D) -> ExternalDoc {
        ExternalDoc {
            url: doc.url().clone(),
            description: doc.description().clone(),
        }
    }
}
