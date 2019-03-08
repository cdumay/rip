use url::Url;

pub trait LicenseGetters {
    fn name(&self) -> &String;
    fn url(&self) -> &Option<Url>;
}


#[derive(Debug)]
pub struct License {
    name: String,
    url: Option<Url>,
}

impl License {
    pub fn new(name: &str, url: Option<&str>) -> License {
        License {
            name: name.to_string(),
            url: match url {
                None => None,
                Some(data) => match Url::parse(data) {
                    Err(_) => None,
                    Ok(url) => Some(url)
                }
            },
        }
    }
}


impl LicenseGetters for License {
    fn name(&self) -> &String { &self.name }
    fn url(&self) -> &Option<Url> { &self.url }
}


#[derive(Debug)]
pub enum OpenSourceLicenses {
    Apache,
    BSD3,
    BSD2,
    GPL,
    LGPL,
    MIT,
    MPL2,
    CDDL,
}

impl From<OpenSourceLicenses> for License {
    fn from(osl: OpenSourceLicenses) -> License {
        match osl {
            OpenSourceLicenses::Apache => License::new("Apache License 2.0", Some("https://opensource.org/licenses/Apache-2.0")),
            OpenSourceLicenses::BSD3 => License::new("BSD 3-Clause 'New' or 'Revised' license", Some("https://opensource.org/licenses/BSD-3-Clause")),
            OpenSourceLicenses::BSD2 => License::new("BSD 2-Clause 'Simplified' or 'FreeBSD' license", Some("https://opensource.org/licenses/BSD-2-Clause")),
            OpenSourceLicenses::GPL => License::new("GNU General Public License (GPL)", Some("https://opensource.org/licenses/gpl-license")),
            OpenSourceLicenses::LGPL => License::new("GNU Library or 'Lesser' General Public License (LGPL)", Some("https://opensource.org/licenses/lgpl-license")),
            OpenSourceLicenses::MIT => License::new("MIT license", Some("https://opensource.org/licenses/MIT")),
            OpenSourceLicenses::MPL2 => License::new("Mozilla Public License 2.0", Some("https://opensource.org/licenses/MPL-2.0")),
            OpenSourceLicenses::CDDL => License::new("Common Development and Distribution License", Some("https://opensource.org/licenses/CDDL-1.0")),
        }
    }
}
