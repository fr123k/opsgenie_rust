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
pub struct ScheduleOverride {
    #[serde(rename = "_parent", skip_serializing_if = "Option::is_none")]
    pub _parent: Option<Box<crate::models::ScheduleMeta>>,
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::Recipient>>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "rotations", skip_serializing_if = "Option::is_none")]
    pub rotations: Option<Vec<crate::models::ScheduleOverrideRotation>>,
}

impl ScheduleOverride {
    pub fn new() -> ScheduleOverride {
        ScheduleOverride {
            _parent: None,
            alias: None,
            user: None,
            start_date: None,
            end_date: None,
            rotations: None,
        }
    }
}


