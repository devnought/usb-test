use nu_table::{NuTable, NuTableConfig, TableTheme};

use vendors::Vendors;
mod vendors;

fn main() -> anyhow::Result<()> {
    let vendors = vendors::vendor_set();
    let iter = nusb::list_devices()?.filter(|d| vendors.contains(&d.vendor_id()));

    let mut rows = vec![vec![
        String::from("Vendor"),
        String::from("Product"),
        String::from("Serial"),
    ]];

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
            let vendor = format!("{vendor:?}");

            let product = if let Some(p) = device_info.product_string() {
                p.to_string()
            } else {
                "N/A".to_string()
            };

            let serial = if let Some(s) = device_info.serial_number() {
                s.to_string()
            } else {
                "N/A".to_string()
            };

            println!("{device_info:#?}");

            let row = vec![vendor, product, serial];

            rows.push(row);
        }
    }

    let mut table = NuTable::new(rows.len(), 3);
    for (row_index, row) in rows.into_iter().enumerate() {
        for (col_index, col) in row.into_iter().enumerate() {
            table.insert((row_index, col_index), col);
        }
    }

    let table_cfg = NuTableConfig {
        theme: TableTheme::rounded(),
        with_header: true,
        ..Default::default()
    };

    let output_table = table.draw(table_cfg, 80).unwrap();
    println!("{output_table}");

    Ok(())
}
