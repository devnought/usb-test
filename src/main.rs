use vendors::Vendors;
mod vendors;

fn main() -> anyhow::Result<()> {
    let vendors = vendors::vendor_set();
    let iter = nusb::list_devices()?.filter(|d| vendors.contains(&d.vendor_id()));

    for device_info in iter {
        let vendor_id = device_info.vendor_id();
        let vendor = Vendors::from_repr(vendor_id).unwrap();

        let Ok(device) = device_info.open().inspect_err(|e| eprintln!("{e}")) else {
            continue;
        };

        let is_android = device
            .configurations()
            .flat_map(|c| c.interface_alt_settings())
            .any(|s| s.class() == 0xff && s.subclass() == 0x42 && s.protocol() == 0x01);

        let is_apple = vendor_id == Vendors::Apple as u16;

        if is_android || is_apple {
            println!(
                "{vendor:?} - {:?} - {:?}",
                device_info.product_string(),
                device_info.serial_number()
            );
        }
    }

    Ok(())
}
