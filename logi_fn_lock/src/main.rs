#![feature(let_chains)]
use anyhow::Result;
use hidapi::{HidApi, HidError};

struct Keyboard {
    pub vendor: u16,
    pub device_id: u16,
    pub fkey_sequence: &'static [u8],
}

const LOGITECH_VENDOR_ID: u16 = 0x046d;

const K380_K480_SEQ: &'static [u8] = &[0x10, 0xff, 0x0b, 0x1e, 0x00, 0x00, 0x00];

const K380_ID: u16 = 0xb342;

const K480_ID: u16 = 0xb33d;

fn main() {
    _main().unwrap();
}

fn _main() -> Result<()> {
    let hid = HidApi::new()?;

    let k380 = Keyboard {
        vendor: LOGITECH_VENDOR_ID,
        device_id: K380_ID,
        fkey_sequence: K380_K480_SEQ,
    };
    let k480 = Keyboard {
        vendor: LOGITECH_VENDOR_ID,
        device_id: K480_ID,
        fkey_sequence: K380_K480_SEQ,
    };

    for (hid_node, device) in [k380, k480].into_iter().filter_map(|keyboard| {
        Some((
            hid.open(keyboard.vendor, keyboard.device_id)
                .map_err(react_to_err)
                .ok()?,
            keyboard,
        ))
    }) {
        let _ = hid_node.write(&device.fkey_sequence);
    }

    Ok(())
}

fn react_to_err(err: HidError) -> HidError {
    #[cfg(target_os = "linux")]
    if let HidError::HidApiError { message } = &err
        && !message.starts_with("No HID devices")
    {
        eprintln!("{err:#?}");
    }
    err
}
