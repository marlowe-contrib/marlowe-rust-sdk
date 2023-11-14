/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenMetadataFile {
    #[serde(rename = "mediaType")]
    pub media_type: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "src")]
    pub src: String,
}

impl TokenMetadataFile {
    pub fn new(media_type: String, name: String, src: String) -> TokenMetadataFile {
        TokenMetadataFile {
            media_type,
            name,
            src,
        }
    }
}


