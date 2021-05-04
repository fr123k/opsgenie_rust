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
pub struct GetScheduleTimelineResponseAllOf {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::ScheduleTimeline>>,
}

impl GetScheduleTimelineResponseAllOf {
    pub fn new() -> GetScheduleTimelineResponseAllOf {
        GetScheduleTimelineResponseAllOf {
            data: None,
        }
    }
}


