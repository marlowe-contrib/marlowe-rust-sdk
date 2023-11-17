/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Assets {
    #[serde(rename = "lovelace")]
    pub lovelace: i32,
    #[serde(rename = "tokens")]
    pub tokens: ::std::collections::HashMap<String, ::std::collections::HashMap<String, i32>>,
}

impl Assets {
    pub fn new(lovelace: i32, tokens: ::std::collections::HashMap<String, ::std::collections::HashMap<String, i32>>) -> Assets {
        Assets {
            lovelace,
            tokens,
        }
    }
}


