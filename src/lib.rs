use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub page: Page,
    pub components: Vec<Component>,
    pub incidents: Vec<Value>,
    #[serde(rename = "scheduled_maintenances")]
    pub scheduled_maintenances: Vec<ScheduledMaintenance>,
    pub status: Status,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: String,
    pub name: String,
    pub url: String,
    #[serde(rename = "time_zone")]
    pub time_zone: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub id: String,
    pub name: String,
    pub status: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub position: i64,
    pub description: Option<String>,
    pub showcase: bool,
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,
    #[serde(rename = "group_id")]
    pub group_id: Option<String>,
    #[serde(rename = "page_id")]
    pub page_id: String,
    pub group: bool,
    #[serde(rename = "only_show_if_degraded")]
    pub only_show_if_degraded: bool,
    #[serde(default)]
    pub components: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledMaintenance {
    pub id: String,
    pub name: String,
    pub status: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "monitoring_at")]
    pub monitoring_at: Value,
    #[serde(rename = "resolved_at")]
    pub resolved_at: Value,
    pub impact: String,
    pub shortlink: String,
    #[serde(rename = "started_at")]
    pub started_at: String,
    #[serde(rename = "page_id")]
    pub page_id: String,
    #[serde(rename = "incident_updates")]
    pub incident_updates: Vec<IncidentUpdate>,
    pub components: Vec<Component2>,
    #[serde(rename = "scheduled_for")]
    pub scheduled_for: String,
    #[serde(rename = "scheduled_until")]
    pub scheduled_until: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncidentUpdate {
    pub id: String,
    pub status: String,
    pub body: String,
    #[serde(rename = "incident_id")]
    pub incident_id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "display_at")]
    pub display_at: String,
    #[serde(rename = "deliver_notifications")]
    pub deliver_notifications: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedComponent {
    pub code: String,
    pub name: String,
    #[serde(rename = "old_status")]
    pub old_status: String,
    #[serde(rename = "new_status")]
    pub new_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component2 {
    pub id: String,
    pub name: String,
    pub status: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub position: i64,
    pub description: Value,
    pub showcase: bool,
    #[serde(rename = "start_date")]
    pub start_date: Value,
    #[serde(rename = "group_id")]
    pub group_id: String,
    #[serde(rename = "page_id")]
    pub page_id: String,
    pub group: bool,
    #[serde(rename = "only_show_if_degraded")]
    pub only_show_if_degraded: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub indicator: String,
    pub description: String,
}
