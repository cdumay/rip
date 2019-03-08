pub trait DocumentationBuilder {
    fn new(url: url::Url) -> Self;
    fn set_description(self, description: String) -> Self;
}

pub trait DocumentationGetters {
    fn url(&self) -> &url::Url;
    fn description(&self) -> &Option<String>;
}

pub trait DocumentationSetters {
    fn description_mut(&mut self) -> &mut Option<String>;
}

#[derive(Debug)]
pub struct Documentation {
    url: url::Url,
    description: Option<String>,
}


impl DocumentationBuilder for Documentation {
    fn new(url: url::Url) -> Documentation {
        Documentation {
            url,
            description: None,
        }
    }
    fn set_description(mut self, description: String) -> Documentation {
        self.description = Some(description);
        self
    }
}

impl DocumentationGetters for Documentation {
    fn url(&self) -> &url::Url { &self.url }
    fn description(&self) -> &Option<String> { &self.description }
}

impl DocumentationSetters for Documentation {
    fn description_mut(&mut self) -> &mut Option<String> { &mut self.description }
}