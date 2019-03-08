use std::collections::BTreeMap;

pub trait ServerVariableBuilder {
    fn new(values: Vec<String>, default: String) -> Self;
    fn set_description(self, description: String) -> Self;
}

pub trait ServerVariableGetters {
    fn values(&self) -> &Vec<String>;
    fn default(&self) -> &String;
    fn description(&self) -> &Option<String>;
}

pub trait ServerVariableSetters {
    fn description_mut(&mut self) -> &mut Option<String>;
}

#[derive(Debug)]
pub struct ServerVariable {
    values: Vec<String>,
    default: String,
    description: Option<String>,
}


impl ServerVariableBuilder for ServerVariable {
    fn new(values: Vec<String>, default: String) -> ServerVariable {
        ServerVariable {
            values,
            default,
            description: None,
        }
    }
    fn set_description(mut self, description: String) -> ServerVariable {
        self.description = Some(description);
        self
    }
}

impl ServerVariableGetters for ServerVariable {
    fn values(&self) -> &Vec<String> { &self.values }
    fn default(&self) -> &String { &self.default }
    fn description(&self) -> &Option<String> { &self.description }
}

impl ServerVariableSetters for ServerVariable {
    fn description_mut(&mut self) -> &mut Option<String> { &mut self.description }
}

pub trait ServerBuilder {
    fn new(url: url::Url) -> Self;
    fn set_description(self, description: String) -> Self;
    fn set_variables(self, variables: BTreeMap<String, ServerVariable>) -> Self;
}

pub trait ServerGetters {
    fn url(&self) -> &url::Url;
    fn description(&self) -> &Option<String>;
    fn variables(&self) -> &BTreeMap<String, ServerVariable>;
}

pub trait ServerSetters {
    fn description_mut(&mut self) -> &mut Option<String>;
    fn variables_mut(&mut self) -> &mut BTreeMap<String, ServerVariable>;
}

#[derive(Debug)]
pub struct Server {
    url: url::Url,
    description: Option<String>,
    variables: BTreeMap<String, ServerVariable>,
}


impl ServerBuilder for Server {
    fn new(url: url::Url) -> Server {
        Server {
            url,
            description: None,
            variables: BTreeMap::new(),
        }
    }
    fn set_description(mut self, description: String) -> Server {
        self.description = Some(description);
        self
    }
    fn set_variables(mut self, variables: BTreeMap<String, ServerVariable>) -> Server {
        self.variables = variables;
        self
    }
}

impl ServerGetters for Server {
    fn url(&self) -> &url::Url { &self.url }
    fn description(&self) -> &Option<String> { &self.description }
    fn variables(&self) -> &BTreeMap<String, ServerVariable> { &self.variables }
}

impl ServerSetters for Server {
    fn description_mut(&mut self) -> &mut Option<String> { &mut self.description }
    fn variables_mut(&mut self) -> &mut BTreeMap<String, ServerVariable> { &mut self.variables }
}