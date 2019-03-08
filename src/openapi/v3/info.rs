use crate::openapi::v3::contact::Contact;
use crate::openapi::v3::license::License;
use crate::rest::application::ApplicationGetters;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Info {
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terms_of_service: Option<String>,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license: Option<License>,
}


impl<A: ApplicationGetters> From<&A> for Info {
    fn from(app: &A) -> Info {
        Info {
            title: app.name().clone(),
            description: app.description().clone(),
            terms_of_service: match app.tos() {
                Some(url) => Some(url.clone().into_string()),
                None => None
            },
            contact: match app.contact() {
                Some(contact) => Some(Contact::from(contact)),
                None => None
            },
            license: match app.license() {
                Some(data) => Some(License::from(data)),
                None => None
            },
            version: app.version().clone(),
        }
    }
}
