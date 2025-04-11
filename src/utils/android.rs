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

    let mut output_string: Vec<_> = String::from_utf8(output.stdout)
        .unwrap()
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    output_string.pop();

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
            .args(["-m", "-d", "emulator", "-avd", emulator_name])
            .output()
            .expect("Failed to start emulator");
    } else {
        Command::new("echo")
            .args(["No support for your OS yet"])
            .output()
            .expect("Failed to execute command");
    }
}

pub fn create_android_emulator(emulator_name: &str, device_identifier: &str, system_image: &str) {
    if cfg!(target_os = "macos") {
        let output = Command::new("avdmanager")
            .args([
                "create",
                "avd",
                "-n",
                emulator_name,
                "-k",
                system_image,
                "-d",
                device_identifier,
            ])
            .output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    eprintln!(
                        "Failed to create Android emulator: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                } else {
                    println!("Android emulator created successfully.");
                }
            }
            Err(e) => {
                eprintln!("Failed to execute command: {}", e);
            }
        }
    } else {
        eprintln!("No support for your OS yet");
    }
}

pub fn delete_android_emulator(emulator_name: &str) {
    if cfg!(target_os = "macos") {
        Command::new("avdmanager")
            .args(["delete", "avd", "--name", emulator_name])
            .output()
            .expect("Failed to delete Android emulator");
    } else {
        Command::new("echo")
            .args(["No support for your OS yet"])
            .output()
            .expect("Failed to execute command");
    }
}

pub fn list_device_profiles() -> Vec<String> {
    let output = Command::new("avdmanager")
        .args(["list", "device"])
        .output()
        .expect("Failed to execute avdmanager");

    let output_str = String::from_utf8_lossy(&output.stdout);

    // Parse the output to extract device profile names
    let mut device_profiles = Vec::new();
    for line in output_str.lines() {
        if line.starts_with("id:") {
            if let Some(name) = line.split('"').nth(1) {
                device_profiles.push(name.to_string());
            }
        }
    }

    device_profiles
}

pub fn list_system_images() -> Vec<String> {
    let output = Command::new("sdkmanager")
        .args(["--list"])
        .output()
        .expect("Failed to execute sdkmanager");

    let output_str = String::from_utf8_lossy(&output.stdout);

    // Parse the output to extract system image identifiers
    let mut system_images = Vec::new();
    for line in output_str.lines() {
        if line.contains("system-images;") {
            if let Some(formatted_image) = format_system_image_path(line) {
                system_images.push(formatted_image);
            }
        }
    }

    system_images
}

fn format_system_image_path(line: &str) -> Option<String> {
    // Example line: "system-images;android-35;google_apis;arm64-v8a"
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() >= 4 {
        let path_parts: Vec<&str> = parts[0].trim().split(';').collect();
        if path_parts.len() == 4 {
            return Some(format!(
                "system-images;{};{};{}",
                path_parts[1].trim(),
                path_parts[2].trim(),
                path_parts[3].trim()
            ));
        }
    }
    None
}
