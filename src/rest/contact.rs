use url::Url;

pub trait ContactBuilder {
    fn new(name: String) -> Self;
    fn set_url(self, url: Url) -> Self;
    fn set_email(self, email: String) -> Self;
}

pub trait ContactGetters {
    fn name(&self) -> &String;
    fn url(&self) -> &Option<Url>;
    fn email(&self) -> &Option<String>;
}

pub trait ContactSetters {
    fn url_mut(&mut self) -> &mut Option<Url>;
    fn email_mut(&mut self) -> &mut Option<String>;
}

#[derive(Debug)]
pub struct Contact {
    name: String,
    url: Option<Url>,
    email: Option<String>,
}


impl ContactBuilder for Contact {
    fn new(name: String) -> Contact {
        Contact {
            name,
            url: None,
            email: None,
        }
    }
    fn set_url(mut self, url: Url) -> Contact {
        self.url = Some(url);
        self
    }
    fn set_email(mut self, email: String) -> Contact {
        self.email = Some(email);
        self
    }
}

impl ContactGetters for Contact {
    fn name(&self) -> &String { &self.name }
    fn url(&self) -> &Option<Url> { &self.url }
    fn email(&self) -> &Option<String> { &self.email }
}

impl ContactSetters for Contact {
    fn url_mut(&mut self) -> &mut Option<Url> { &mut self.url }
    fn email_mut(&mut self) -> &mut Option<String> { &mut self.email }
}