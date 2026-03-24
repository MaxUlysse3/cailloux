use std::io::Result;
use evdev::{Device};

pub fn get_stylus() -> Device {
    let mut styluses = evdev::enumerate().filter(|(_, dev)| dev.name().unwrap_or("").to_lowercase().contains("stylus")).collect::<Vec<_>>();
    if styluses.len() > 1 {
        panic!("Multiple tablets detected.");
    } else if styluses.len() == 0 {
        panic!("No styluses detected.");
    } else {
        styluses.pop().unwrap().1
    }
}

