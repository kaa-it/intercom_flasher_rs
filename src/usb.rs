#[cfg(test)]
mod tests {
    use super::*;
    use libparted::*;

    #[test]
    fn print_devices() {
        for device in Device::devices(true) {
            println!("{}", device.path().to_str().unwrap())
        }
    }
}