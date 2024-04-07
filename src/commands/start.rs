use crate::structs::android_devices::Device;
use crate::utils::android::start_android_emulator;
use crate::utils::ios::start_ios_simulator;
use crate::{structs::ios_devices::Devices, utils::ios::extract_ios_version};
use dialoguer::{theme::ColorfulTheme, Select};

pub fn run(ios_devices: Devices, android_devices: Vec<Device>, ios: bool, android: bool) {
    if ios {
        select_ios_device(ios_devices);
    } else if android {
        select_android_device(android_devices);
    } else {
        let platform = select_platform().unwrap();
        match platform.as_str() {
            "iOS" => select_ios_device(ios_devices),
            "Android" => select_android_device(android_devices),
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

fn select_ios_device(ios_devices: Devices) {
    println!("Selecting iOS device");

    let mut selections = Vec::new();
    for (platform, device_list) in ios_devices.devices.iter() {
        let ios_version = extract_ios_version(platform).unwrap_or_default();
        device_list.iter()
            .for_each(|device| {
                let display_name = if ios_version.is_empty() {
                    device.name.clone()
                } else {
                    format!("{} ({})", device.name, ios_version)
                };
                selections.push((display_name.clone(), device.udid.clone()));
            });
    }

    let device_names: Vec<&str> = selections.iter().map(|(name, _)| name.as_str()).collect();

    let selection_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your iOS device")
        .default(0)
        .items(&device_names)
        .interact()
        .unwrap();

    let selected_device_identifier = &selections[selection_index].1;
    start_ios_simulator(&selected_device_identifier);
}

fn select_android_device(android_devices: Vec<Device>) {
    println!("Selecting Android device");

    let mut selections = Vec::new();
    for device in android_devices {
        selections.push(device.name.clone());
    }

    let selection_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your Android device")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let selected_device_identifier = &selections[selection_index];
    start_android_emulator(&selected_device_identifier);
}
