use crate::structs::devices::Devices;

pub fn run(devices: Devices) {
    for (_, device) in devices.devices.iter() {
        for device in device {
                println!("{} - {}", device.name, device.udid);
        }
    }
}
