use anyhow::Result;
use hidapi::HidApi;

struct Keyboard {
    pub vendor: u16,
    pub device_id: u16,
    pub fkey_sequence: Vec<u8>,
}

const LOGITECH_VENDOR_ID: u16 = 0x046d;

fn main() {
    _main().unwrap();
}

fn _main() -> Result<()> {
    let hid = HidApi::new()?;
    let vec: Vec<Keyboard> = vec![
        // K380
        Keyboard {
            vendor: LOGITECH_VENDOR_ID,
            device_id: 0xb342,
            fkey_sequence: vec![0x10, 0xff, 0x0b, 0x1e, 0x00, 0x00, 0x00],
        },
        // K480
        Keyboard {
            vendor: LOGITECH_VENDOR_ID,
            device_id: 0xb33d,
            fkey_sequence: vec![0x10, 0xff, 0x0b, 0x1e, 0x00, 0x00, 0x00],
        },
    ];
    for (device, sequence) in vec.into_iter().filter_map(|d| {
        Some((
            hid.open(d.vendor, d.device_id)
                .map_err(|err| println!("{err:#?}"))
                .ok()?,
            d.fkey_sequence,
        ))
    }) {
        device.write(&sequence)?;
    }
    Ok(())
}
