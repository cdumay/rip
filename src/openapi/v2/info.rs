use crate::openapi::v2::contact::Contact;
use crate::openapi::v2::license::License;
use crate::rest::application::ApplicationGetters;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Info {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terms_of_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact: Option<Vec<Contact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license: Option<License>,
    version: Option<String>,
}


impl<A: ApplicationGetters> From<&A> for Info {
    fn from(app: &A) -> Info {
        Info {
            title: Some(app.name().clone()),
            description: app.description().clone(),
            terms_of_service: match app.tos() {
                Some(url) => Some(url.clone().into_string()),
                None => None
            },
            contact: match app.contact() {
                Some(contact) => Some(vec![Contact::from(contact)]),
                None => None
            },
            license: match app.license() {
                Some(data) => Some(License::from(data)),
                None => None
            },
            version: Some(app.version().clone()),
        }
    }
}
