pub mod tablet_handler;
pub mod sign_reader;

use std::thread;
use std::time::{Duration};
use std::sync::mpsc;

use tablet_handler::{Stylus, StylusData};

use hyprland::keyword::{Keyword, OptionValue};

use evdev::{Device, EventType, KeyCode, AbsoluteAxisCode};

fn main() {
    let mut styluses = evdev::enumerate().filter(|(_, dev)| dev.name().unwrap_or("").to_lowercase().contains("stylus")).collect::<Vec<_>>();
    let mut s1 = if styluses.len() > 1 {
        panic!("Multiple tablets detected.");
    } else if styluses.len() == 0 {
        panic!("No styluses detected.");
    } else {
        styluses.pop().unwrap().1
    };

    let mut signs = std::collections::HashMap::<u128, Box<dyn Fn()>>::new();

    signs.insert(4125, Box::new(|| {
        let name = "input:touchdevice:enabled";
        let opt = Keyword::get(name);
        match opt {
            Ok(o) => {
                match o.value {
                    OptionValue::Int(v) => {
                        let new_val = format!("{}", (1 - v));
                        let _ = Keyword::set(name, new_val);
                    },
                    _ => println!("Unable to access state of `input:touchdevice:enabled`"),
                }
            },
            _ => ()
        }
    }));

    let mut x = 0;
    let mut y = 0;

    loop {
        for e in s1.fetch_events().expect("Error fetching events from thread 1.") {
            match e.event_type() {
                EventType::KEY => {
                    if e.code() == KeyCode::BTN_STYLUS.code() && e.value() == 1 {
                        println!("Button pressed from thread 1!");
                        sign_reader::start_reading_signs((x, y), 2000, &signs);
                    }
                },
                EventType::ABSOLUTE => {
                    const ABS_X: u16 = AbsoluteAxisCode::ABS_X.0;
                    const ABS_Y: u16 = AbsoluteAxisCode::ABS_Y.0;
                    match e.code() {
                        ABS_X => x = e.value(),
                        ABS_Y => y = e.value(),
                        _ => ()
                    }
                },
                _ => (),
            }
        }
    }
}
