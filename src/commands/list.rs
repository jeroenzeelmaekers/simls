use crate::structs::devices::Devices;
use crate::utils::extract_ios_version;
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
    for (platform, device) in devices.devices.iter() {
        if let Some(ios_version) = extract_ios_version(platform) {
            device.iter().for_each(|device| {
                let status = match device.state.as_str() {
                    "Booted" => device.state.green(),
                    "Shutdown" => device.state.red(),
                    _ => device.state.normal(),
                };
                println!("{} ({}) - {}", device.name, ios_version, status)
            });
        } else {
            device.iter().for_each(|device| {
                let status = match device.state.as_str() {
                    "Booted" => device.state.green(),
                    "Shutdown" => device.state.red(),
                    _ => device.state.normal(),
                };
                println!("{} - {}", device.name, status)
            });
        }
    }
}

fn list_android(android_devices: Vec<String>) {
    println!("Android Devices:");

    android_devices
        .iter()
        .for_each(|device| println!("{}", device));
}
