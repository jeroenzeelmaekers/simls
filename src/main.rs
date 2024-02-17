use std::process::Command;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct Device {
    #[serde(rename = "lastBootedAt")] // Renaming to match the field name in JSON
    last_booted_at: Option<String>, // Optional because it's not present in all devices
    #[serde(rename = "dataPath")] // Renaming to match the field name in JSON
    data_path: String,
    #[serde(rename = "dataPathSize")] // Renaming to match the field name in JSON
    data_path_size: u64,
    #[serde(rename = "logPath")] // Renaming to match the field name in JSON
    log_path: String,
    #[serde(rename = "udid")] // Renaming to match the field name in JSON
    udid: String,
    #[serde(rename = "isAvailable")] // Renaming to match the field name in JSON
    is_available: bool,
    #[serde(rename = "logPathSize")] // Renaming to match the field name in JSON
    log_path_size: Option<u64>,      // Optional because it's not present in all devices
    #[serde(rename = "deviceTypeIdentifier")] // Renaming to match the field name in JSON
    device_type_identifier: String,
    #[serde(rename = "state")] // Renaming to match the field name in JSON
    state: String,
    #[serde(rename = "name")] // Renaming to match the field name in JSON
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Devices {
    devices: HashMap<String, Vec<Device>>,
}
fn main() -> Result<()>  {
    let output = if cfg!(target_os = "macos") {
        Command::new("xcrun")
            .args(["simctl", "list", "--json", "devices"])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new("echo")
            .args(["No support for your OS yet"])
            .output()
            .expect("Failed to execute command")
    };

    let output_string = String::from_utf8(output.stdout).unwrap();

    let devices: Devices = serde_json::from_str(&output_string)?;

    for (key, value) in devices.devices.iter() {
        println!("Key: {}", key);
        for device in value {
            println!("Device: {:?}", device.name);
        }
    }

    Ok(())
}
