use libparted::{Device, Disk};

pub fn get_devices() -> Vec<[String;3]> {
    let mut descriptions: Vec<[String;3]> = vec![];

    for mut device in Device::devices(true) {
        if let Ok(()) = device.open() {
            let description = [
                device.path().to_str().unwrap().to_string(),
                device.model().to_string(),
                format!("{:.1}", device.length() as f64 * 512.0 / 1024.0 / 1024.0 / 1024.0),
            ];

            descriptions.push(description);
        }

    }

    descriptions
}


#[cfg(test)]
mod tests {
    use super::*;
    use libparted::{Device, Disk};

    #[test]
    fn print_devices() {
        for device in Device::devices(true) {
            println!("{}", device.path().to_str().unwrap());
            println!("{}", device.model());
            println!("{}", device.length() as f64 * 512.0 / 1024.0 / 1024.0 / 1024.0);
        }
    }

    #[test]
    fn print_device() {
        if let Ok(mut device) = Device::get("/dev/sda") {
            let disk = Disk::new(&mut device);

            if let Ok(disk) = disk {
                for part in disk.parts() {
                    // if part.num() < 1 {
                    //     continue
                    // }

                    println!("    Part {}", part.num());
                    println!("        Type Name: {:?}", part.type_get_name());
                    println!("        Name:      {:?}", part.name());
                    println!("        Path:      {:?}", part.get_path());
                    println!("        Active:    {}", part.is_active());
                    println!("        Busy:      {}", part.is_busy());
                    println!("        FS:        {:?}", part.fs_type_name());
                    println!("        Start:     {}", part.geom_start());
                    println!("        End:       {}", part.geom_end());
                    println!("        Length:    {}", part.geom_length());

                    println!("========================")
                }
            }
        }
    }
}