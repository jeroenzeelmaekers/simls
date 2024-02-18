use crate::structs::devices::Devices;
use crate::utils::ios::extract_ios_version;
use colored::Colorize;

pub fn run(ios_devices: Devices, android_devices: Vec<String>, ios: bool, android: bool) {
    if ios {
        list_ios(ios_devices);
    } else if android {
        list_android(android_devices);
    } else {
        list_ios(ios_devices);
        list_android(android_devices);
    }
}

fn list_ios(devices: Devices) {
    println!("iOS Devices:");
    for (platform, device_list) in devices.devices.iter() {
        let ios_version = extract_ios_version(platform).unwrap_or_default();
        for device in device_list {
            let status = match device.state.as_str() {
                "Booted" => device.state.green(),
                "Shutdown" => device.state.red(),
                _ => device.state.normal(),
            };
            let display_name = if ios_version.is_empty() {
                device.name.clone()
            } else {
                format!("{} ({})", device.name, ios_version)
            };
            println!("{} - {}", display_name, status);
        }
    }
}

fn list_android(android_devices: Vec<String>) {
    println!("Android Devices:");
    android_devices
        .iter()
        .for_each(|device| println!("{}", device));
}
