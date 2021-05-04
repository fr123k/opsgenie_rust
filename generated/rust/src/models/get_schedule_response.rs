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
pub struct GetScheduleResponse {
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "took")]
    pub took: f32,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::Schedule>>,
}

impl GetScheduleResponse {
    pub fn new(request_id: String, took: f32) -> GetScheduleResponse {
        GetScheduleResponse {
            request_id,
            took,
            data: None,
        }
    }
}


