use crate::rest::contact::Contact;
use crate::rest::documentation::Documentation;
use crate::rest::license::License;
use crate::rest::scheme::Scheme;
use crate::rest::server::Server;

pub trait ApplicationBuilder {
    fn new(name: String, version: String) -> Self;
    fn set_base_url(self, base_url: String) -> Self;
    fn set_scheme(self, scheme: Vec<Scheme>) -> Self;
    fn set_consumes(self, consumes: Vec<mime::Mime>) -> Self;
    fn set_produces(self, produces: Vec<mime::Mime>) -> Self;
    fn set_description(self, description: String) -> Self;
    fn set_tos(self, tos: url::Url) -> Self;
    fn set_contact(self, contact: Contact) -> Self;
    fn set_license(self, license: License) -> Self;
    fn set_servers(self, servers: Vec<Server>) -> Self;
    fn set_documentation(self, documentation: Documentation) -> Self;
}

pub trait ApplicationGetters {
    fn name(&self) -> &String;
    fn version(&self) -> &String;
    fn base_url(&self) -> &Option<String>;
    fn scheme(&self) -> &Option<Vec<Scheme>>;
    fn consumes(&self) -> &Option<Vec<mime::Mime>>;
    fn produces(&self) -> &Option<Vec<mime::Mime>>;
    fn description(&self) -> &Option<String>;
    fn tos(&self) -> &Option<url::Url>;
    fn contact(&self) -> &Option<Contact>;
    fn license(&self) -> &Option<License>;
    fn servers(&self) -> &Vec<Server>;
    fn documentation(&self) -> &Option<Documentation>;
}

pub trait ApplicationSetters {
    fn base_url_mut(&mut self) -> &mut Option<String>;
    fn scheme_mut(&mut self) -> &mut Option<Vec<Scheme>>;
    fn consumes_mut(&mut self) -> &mut Option<Vec<mime::Mime>>;
    fn produces_mut(&mut self) -> &mut Option<Vec<mime::Mime>>;
    fn description_mut(&mut self) -> &mut Option<String>;
    fn tos_mut(&mut self) -> &mut Option<url::Url>;
    fn contact_mut(&mut self) -> &mut Option<Contact>;
    fn license_mut(&mut self) -> &mut Option<License>;
    fn servers_mut(&mut self) -> &mut Vec<Server>;
    fn documentation_mut(&mut self) -> &mut Option<Documentation>;
}

#[derive(Debug)]
pub struct Application {
    name: String,
    version: String,
    base_url: Option<String>,
    scheme: Option<Vec<Scheme>>,
    consumes: Option<Vec<mime::Mime>>,
    produces: Option<Vec<mime::Mime>>,
    description: Option<String>,
    tos: Option<url::Url>,
    contact: Option<Contact>,
    license: Option<License>,
    servers: Vec<Server>,
    documentation: Option<Documentation>,
}


impl ApplicationBuilder for Application {
    fn new(name: String, version: String) -> Application {
        Application {
            name,
            version,
            base_url: None,
            scheme: None,
            consumes: None,
            produces: None,
            description: None,
            tos: None,
            contact: None,
            license: None,
            servers: Vec::new(),
            documentation: None,
        }
    }
    fn set_base_url(mut self, base_url: String) -> Application {
        self.base_url = Some(base_url);
        self
    }
    fn set_scheme(mut self, scheme: Vec<Scheme>) -> Application {
        self.scheme = Some(scheme);
        self
    }
    fn set_consumes(mut self, consumes: Vec<mime::Mime>) -> Application {
        self.consumes = Some(consumes);
        self
    }
    fn set_produces(mut self, produces: Vec<mime::Mime>) -> Application {
        self.produces = Some(produces);
        self
    }
    fn set_description(mut self, description: String) -> Application {
        self.description = Some(description);
        self
    }
    fn set_tos(mut self, tos: url::Url) -> Application {
        self.tos = Some(tos);
        self
    }
    fn set_contact(mut self, contact: Contact) -> Application {
        self.contact = Some(contact);
        self
    }
    fn set_license(mut self, license: License) -> Application {
        self.license = Some(license);
        self
    }
    fn set_servers(mut self, servers: Vec<Server>) -> Application {
        self.servers = servers;
        self
    }
    fn set_documentation(mut self, documentation: Documentation) -> Application {
        self.documentation = Some(documentation);
        self
    }
}

impl ApplicationGetters for Application {
    fn name(&self) -> &String { &self.name }
    fn version(&self) -> &String { &self.version }
    fn base_url(&self) -> &Option<String> { &self.base_url }
    fn scheme(&self) -> &Option<Vec<Scheme>> { &self.scheme }
    fn consumes(&self) -> &Option<Vec<mime::Mime>> { &self.consumes }
    fn produces(&self) -> &Option<Vec<mime::Mime>> { &self.produces }
    fn description(&self) -> &Option<String> { &self.description }
    fn tos(&self) -> &Option<url::Url> { &self.tos }
    fn contact(&self) -> &Option<Contact> { &self.contact }
    fn license(&self) -> &Option<License> { &self.license }
    fn servers(&self) -> &Vec<Server> { &self.servers }
    fn documentation(&self) -> &Option<Documentation> { &self.documentation }
}

impl ApplicationSetters for Application {
    fn base_url_mut(&mut self) -> &mut Option<String> { &mut self.base_url }
    fn scheme_mut(&mut self) -> &mut Option<Vec<Scheme>> { &mut self.scheme }
    fn consumes_mut(&mut self) -> &mut Option<Vec<mime::Mime>> { &mut self.consumes }
    fn produces_mut(&mut self) -> &mut Option<Vec<mime::Mime>> { &mut self.produces }
    fn description_mut(&mut self) -> &mut Option<String> { &mut self.description }
    fn tos_mut(&mut self) -> &mut Option<url::Url> { &mut self.tos }
    fn contact_mut(&mut self) -> &mut Option<Contact> { &mut self.contact }
    fn license_mut(&mut self) -> &mut Option<License> { &mut self.license }
    fn servers_mut(&mut self) -> &mut Vec<Server> { &mut self.servers }
    fn documentation_mut(&mut self) -> &mut Option<Documentation> { &mut self.documentation }
}