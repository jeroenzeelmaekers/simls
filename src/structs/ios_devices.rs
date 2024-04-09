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

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceType {
    #[serde(rename = "productFamily")]
    pub product_family: String,
    #[serde(rename = "bundlePath")]
    pub bundle_path: String,
    #[serde(rename = "maxRuntimeVersion")]
    pub max_runtime_version: String,
    #[serde(rename = "maxRuntimeVersionString")]
    pub max_runtime_version_string: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "minRuntimeVersion")]
    pub min_runtime_version: String,
    #[serde(rename = "minRuntimeVersionString")]
    pub min_runtime_version_string: String,
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceTypes {
    #[serde(rename = "devicetypes")]
    pub device_types: Vec<DeviceType>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SupportedDeviceTypes {
    #[serde(rename = "bundlePath")]
    pub bundle_path: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "productFamily")]
    pub product_family: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Runtimes {
    #[serde(rename = "runtimes")]
    pub runtimes: Vec<Runtime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Runtime {
    #[serde(rename = "bundlePath")]
    pub bundle_path: String,
    #[serde(rename = "buildversion")]
    pub build_version: String,
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "runtimeRoot")]
    pub runtime_root: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "isInternal")]
    pub is_internal: bool,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "supportedDeviceTypes")]
    pub supported_device_types: Vec<SupportedDeviceTypes>,
}
