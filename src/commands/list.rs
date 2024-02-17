use crate::structs::devices::Devices;
use crate::utils::extract_ios_version;
use colored::Colorize;

pub fn run(devices: Devices, ios: bool, android: bool) {
    if ios {
        list_ios(devices);
    } else if android {
        list_android();
    } else {
        list_all(devices)
    }
}

fn list_all(ios_devices: Devices) {
    list_ios(ios_devices);
    list_android();
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

fn list_android() {
    println!("Android Devices:");
    println!("list android");
}
