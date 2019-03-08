use crate::rest::contact::ContactGetters;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", with = "url_serde")]
    url: Option<url::Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}


impl<C: ContactGetters> From<&C> for Contact {
    fn from(contact: &C) -> Contact {
        Contact {
            name: Some(contact.name().clone()),
            url: contact.url().clone(),
            email: contact.email().clone(),
        }
    }
}
