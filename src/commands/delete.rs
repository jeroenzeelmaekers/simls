
use dialoguer::{theme::ColorfulTheme, Select};

use crate::{structs::{android_devices::Device, ios_devices::Devices}, utils::ios::{self, extract_ios_version}};

pub fn run(ios_devices: Devices, android_devices: Vec<Device>, ios: bool, android: bool) {
    if ios {
        delete_ios_device(ios_devices);
    } else if android {
        delete_android_device(android_devices);
    } else {
        let platform = select_platform().unwrap();
        match platform.as_str() {
            "iOS" => delete_ios_device(ios_devices),
            "Android" => delete_android_device(android_devices),
            _ => println!("Invalid platform"),
        }
    }
}

fn select_platform() -> Result<String, String> {
    let selections = &["iOS", "Android"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your platform")
        .default(0)
        .items(&selections[..])
        .interact_opt()
        .unwrap();

    match selection {
        Some(selection) => Ok(selections[selection].to_string()),
        None => Err("No platform selected".to_string()),
    }
}

fn delete_ios_device(ios_devices: Devices) {
    println!("Delete iOS device");

    let mut devices_selection = Vec::new();

    for (platform, device_list) in ios_devices.devices.iter() {
        let ios_version = extract_ios_version(platform).unwrap_or_default();
        for device in device_list {
            let device_name = format!("{} ({})", device.name, ios_version);
            devices_selection.push((device_name, device.udid.clone()));
        }
    }

    
    let device_names: Vec<&str> = devices_selection.iter().map(|(name, _)| name.as_str()).collect();
    let selection_device_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your device type")
        .default(0)
        .items(&device_names)
        .interact()
        .unwrap();

    let (_, udid) = devices_selection.get(selection_device_index).unwrap();

    ios::delete_ios_device(udid)
}

fn delete_android_device(_android_devices: Vec<Device>) {
    println!("Delete Android device");
}
