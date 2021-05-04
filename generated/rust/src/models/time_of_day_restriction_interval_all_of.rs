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
pub struct TimeOfDayRestrictionIntervalAllOf {
    #[serde(rename = "restriction", skip_serializing_if = "Option::is_none")]
    pub restriction: Option<Box<crate::models::TimeOfDayRestriction>>,
}

impl TimeOfDayRestrictionIntervalAllOf {
    pub fn new() -> TimeOfDayRestrictionIntervalAllOf {
        TimeOfDayRestrictionIntervalAllOf {
            restriction: None,
        }
    }
}


