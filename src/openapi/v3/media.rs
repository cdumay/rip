use std::collections::BTreeMap;

use crate::openapi::v3::component::ObjectOrReference;
use crate::openapi::v3::encoding::Encoding;
use crate::openapi::v3::example::Example;
use crate::openapi::v3::schema::Schema;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum MediaTypeExample {
    Example {
        example: String
    },
    Examples {
        examples: Option<BTreeMap<String, ObjectOrReference<Example>>>,
    },
}


#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MediaType {
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<ObjectOrReference<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    examples: Option<MediaTypeExample>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<BTreeMap<String, Encoding>>,
}



