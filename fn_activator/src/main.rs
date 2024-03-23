#![feature(let_chains)]
use anyhow::Result;
use clap::Parser;
use hidapi::HidApi;
mod args;
use args::Args;
use regex_static::static_regex;

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

#[cfg(target_os = "linux")]
fn device_finder(device: String) -> Result<Keyboard> {
    use std::{fs::read_link, io};

    let target = read_link(format!("/dev/{device}"))?
        .as_os_str()
        .to_str()
        .ok_or(io::Error::new(io::ErrorKind::NotFound, ""))?
        .to_string();
    #[allow(non_upper_case_globals)]
    let (vendor, device) = if let Some(captures) =
        static_regex!(r"([0-9A-F]{4}):([0-9A-F]{4})\.[0-9A-F]{4}/hidraw").captures(&target)
        && let [_, vendor, device] = captures.iter().filter_map(|c| c).collect::<Vec<_>>()[..]
    {
        Ok((vendor.as_str(), device.as_str()))
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "The hidraw device provided does not have a valid device id",
        ))
    }?;
    let (vendor, device) = (
        u16::from_str_radix(vendor, 16)?,
        u16::from_str_radix(device, 16)?,
    );
    let fkey_sequence = match (vendor, device) {
        (LOGITECH_VENDOR_ID, K380_ID) => Ok(&K380_K480_SEQ),
        (LOGITECH_VENDOR_ID, K480_ID) => Ok(&K380_K480_SEQ),
        _ => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "The device provided is not supported",
        )),
    }?;
    Ok(Keyboard {
        vendor,
        device_id: device,
        fkey_sequence,
    })
}

#[cfg(target_os = "macos")]
fn device_finder(device: String) -> Result<Keyboard> {}

#[cfg(target_os = "windows")]
fn device_finder(device: String) -> Result<Keyboard> {}

fn _main() -> Result<()> {
    let args = Args::parse();

    let device = device_finder(args.device)?;

    HidApi::new()?
        .open(device.vendor, device.device_id)?
        .write(&device.fkey_sequence)?;

    Ok(())
}
