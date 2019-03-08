use crate::rest::license::LicenseGetters;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct License {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", with = "url_serde")]
    url: Option<url::Url>,
}


impl<L: LicenseGetters> From<&L> for License {
    fn from(license: &L) -> License {
        License {
            name: Some(license.name().clone()),
            url: license.url().clone(),
        }
    }
}
