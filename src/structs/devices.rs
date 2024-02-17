use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Device {
    #[serde(rename = "lastBootedAt")]
    pub last_booted_at: Option<String>,
    #[serde(rename = "dataPath")]
    pub data_path: String,
    #[serde(rename = "dataPathSize")]
    pub data_path_size: u64,
    #[serde(rename = "logPath")]
    pub log_path: String,
    #[serde(rename = "udid")]
    pub udid: String,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "logPathSize")]
    pub log_path_size: Option<u64>,
    #[serde(rename = "deviceTypeIdentifier")]
    pub device_type_identifier: String,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Devices {
    pub devices: HashMap<String, Vec<Device>>,
}
