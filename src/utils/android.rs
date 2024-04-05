use serde_json::Result;
use std::process::Command;

use crate::structs::android_devices::Device;

pub fn read_android_emulators() -> Result<Vec<Device>> {
    let output = if cfg!(target_os = "macos") {
        Command::new("emulator")
            .args(["-list-avds"])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new("echo")
            .args(["No support for your OS yet"])
            .output()
            .expect("Failed to execute command")
    };

    let output_string: Vec<_> = String::from_utf8(output.stdout)
        .unwrap()
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    let devices = output_string
        .iter()
        .map(|s| Device {
            name: s.to_string(),
            id: s.to_string(),
        })
        .collect();

    Ok(devices)
}

pub fn start_android_emulator(emulator_name: &str) {
    if cfg!(target_os = "macos") {
        Command::new("screen")
            .args(["-m","-d","emulator","-avd", emulator_name])
            .output()
            .expect("Failed to start emulator");
    } else {
        Command::new("echo")
            .args(["No support for your OS yet"])
            .output()
            .expect("Failed to execute command");
    }
}
