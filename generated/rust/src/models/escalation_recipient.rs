/*
 * Opsgenie REST API
 *
 * Opsgenie OpenAPI Specification
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EscalationRecipient : Escalation recipient



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EscalationRecipient {
    #[serde(rename = "type")]
    pub _type: TypeEm,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl EscalationRecipient {
    /// Escalation recipient
    pub fn new(_type: TypeEm) -> EscalationRecipient {
        EscalationRecipient {
            _type,
            id: None,
            name: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeEm {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "escalation")]
    Escalation,
    #[serde(rename = "schedule")]
    Schedule,
    #[serde(rename = "team")]
    Team,
    #[serde(rename = "group")]
    Group,
}

