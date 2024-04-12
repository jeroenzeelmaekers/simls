use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::{
    structs::{android_devices::Device, ios_devices::Devices},
    utils::ios::{self},
};

pub fn run(ios_devices: Devices, android_devices: Vec<Device>, ios: bool, android: bool) {
    if ios {
        create_ios_device(ios_devices);
    } else if android {
        create_android_device(android_devices);
    } else {
        let platform = select_platform().unwrap();
        match platform.as_str() {
            "iOS" => create_ios_device(ios_devices),
            "Android" => create_android_device(android_devices),
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

fn create_ios_device(_ios_devices: Devices) {
    let ios_runtimes = ios::read_downloaded_ios_version().unwrap();

    let mut runtime_selection = Vec::new();
    for runtime in ios_runtimes.runtimes.iter() {
        runtime_selection.push((runtime.version.clone(), runtime.identifier.clone()));
    }

    let runtime_names: Vec<&str> = runtime_selection
        .iter()
        .map(|(name, _)| name.as_str())
        .collect();
    let selection_runtime_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your runtime")
        .default(0)
        .items(&runtime_names)
        .interact()
        .unwrap();

    let supported_devices = &ios_runtimes
        .runtimes
        .get(selection_runtime_index)
        .unwrap()
        .supported_device_types;

    let mut devices_selection = Vec::new();
    for device in supported_devices.iter() {
        devices_selection.push((device.name.clone(), device.identifier.clone()));
    }

    let device_names: Vec<&str> = devices_selection
        .iter()
        .map(|(name, _)| name.as_str())
        .collect();
    let selection_device_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your device type")
        .default(0)
        .items(&device_names)
        .interact()
        .unwrap();

    let (runtime_name, runtime_identifier) =
        runtime_selection.get(selection_runtime_index).unwrap();
    let (device_name, device_identifier) = devices_selection.get(selection_device_index).unwrap();
    let simulator_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the simulator name")
        .default(format!("{} {}", device_name, runtime_name))
        .interact()
        .unwrap();

    ios::create_ios_device(&simulator_name, &device_identifier, &runtime_identifier);
}

fn create_android_device(_android_devices: Vec<Device>) {
    println!("Create Android device");
    todo!("creating logic for android devices not yet available")
}
