use crate::structs::{android_devices::Device, ios_devices::Devices};

pub fn run(ios_devices: Devices, android_devices: Vec<Device>) {}

// fn select_device() -> Result<String, String> {
//     let selection = Select::with_theme(&ColorfulTheme::default())
//         .with_prompt("Select your platform")
//         .default(0)
//         .items(&selections[..])
//         .interact_opt()
//         .unwrap();
//
//     match selection {
//         Some(selection) => Ok(selections[selection].to_string()),
//         None => Err("No platform selected".to_string()),
//     }
// }
