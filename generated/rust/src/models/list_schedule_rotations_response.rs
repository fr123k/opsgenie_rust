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
pub struct ListScheduleRotationsResponse {
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "took")]
    pub took: f32,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::ScheduleRotation>>,
}

impl ListScheduleRotationsResponse {
    pub fn new(request_id: String, took: f32) -> ListScheduleRotationsResponse {
        ListScheduleRotationsResponse {
            request_id,
            took,
            data: None,
        }
    }
}


