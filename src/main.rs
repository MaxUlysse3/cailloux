pub mod tablet_handler;
pub mod sign_reader;
pub mod commands;

use std::collections::{HashMap};

use hyprland::keyword::{Keyword, OptionValue};

use evdev::{Device, EventType, KeyCode, AbsoluteAxisCode};

fn main() {
    let signs = commands::gen_signs();

    start_listening(&signs);
}

/// Start listenings to the stylus events and dispatch the signs made to the corresponding
/// procedures according to `signs`.
fn start_listening(signs: &HashMap<u128, Box<dyn Fn()>>) -> ! {
    let mut s1 = tablet_handler::get_stylus();

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
