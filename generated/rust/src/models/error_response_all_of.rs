/*
 * Opsgenie REST API
 *
 * Opsgenie OpenAPI Specification
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponseAllOf {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(rename = "responseHeaders", skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<::std::collections::HashMap<String, Vec<String>>>,
}

impl ErrorResponseAllOf {
    pub fn new() -> ErrorResponseAllOf {
        ErrorResponseAllOf {
            message: None,
            code: None,
            response_headers: None,
        }
    }
}


