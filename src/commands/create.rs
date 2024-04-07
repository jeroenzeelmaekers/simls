use dialoguer::{theme::ColorfulTheme, Select};

use crate::structs::{android_devices::Device, ios_devices::Devices};

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
    println!("Create iOS device");
}

fn create_android_device(_android_devices: Vec<Device>) {
    println!("Create Android device");
}
