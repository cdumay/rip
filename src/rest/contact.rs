pub trait ContactBuilder {
    fn new(name: String) -> Self;
    fn set_url(self, url: url::Url) -> Self;
    fn set_email(self, email: String) -> Self;
}

pub trait ContactGetters {
    fn name(&self) -> &String;
    fn url(&self) -> &Option<url::Url>;
    fn email(&self) -> &Option<String>;
}

pub trait ContactSetters {
    fn url_mut(&mut self) -> &mut Option<url::Url>;
    fn email_mut(&mut self) -> &mut Option<String>;
}

#[derive(Debug)]
pub struct Contact {
    name: String,
    url: Option<url::Url>,
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
    fn set_url(mut self, url: url::Url) -> Contact {
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
    fn url(&self) -> &Option<url::Url> { &self.url }
    fn email(&self) -> &Option<String> { &self.email }
}

impl ContactSetters for Contact {
    fn url_mut(&mut self) -> &mut Option<url::Url> { &mut self.url }
    fn email_mut(&mut self) -> &mut Option<String> { &mut self.email }
}