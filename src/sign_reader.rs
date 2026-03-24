use std::collections::{HashMap};
use std::thread;

use crate::tablet_handler::{get_stylus};

use evdev::{Device, EventType, KeyCode, AbsoluteAxisCode};

use hyprland::data::{Monitor, Transforms};
use hyprland::shared::{HyprDataActive};

/// Begin reading the sign that is being made.
pub fn start_reading_signs(center: (i32, i32), cell_size: u32, signs: &HashMap<u128, Box<dyn Fn()>>) {
    let clos = move || {
        println!("Started reading signs! (center : {center:?})");
        let mut stylus = get_stylus();
        let mut active_sign: u128 = 4;
        let mut last_cell = 4;
        let mut x = 0;
        let mut y = 1;
        let mut counter = 0;
        let mut moved = |x, y| {
            // println!("Move event registered!!");
            let transform = Monitor::get_active().expect("Could not read monitor data.").transform;

            let rotation = match transform {
                Transforms::Normal => 0,
                Transforms::Normal90 => 1,
                Transforms::Normal180 => 2,
                Transforms::Normal270 => 3,
                _ => panic!("Symmetry transforms are not yet supported."),
            };

            let cell = get_cell_relative(x, y, cell_size, rotation);
            // println!("Cell : {cell:?} (last : {last_cell:?})");
            // println!("position : {x:?}, {y:?}");
            if cell != last_cell {
                // println!("Cell : {cell:?} (last : {last_cell:?})");
                // println!("position : {x:?}, {y:?}");
                counter += 1;
                // println!("Detected a cell change (nb. {counter:?}), (active_sign: {active_sign:?})");
                active_sign = match active_sign.checked_mul(10) {
                    None => active_sign,
                    Some(val) => val + cell as u128,
                };
                last_cell = cell;
                // println!("changed cell! {:?}", cell);
            }
        };
        const ABS_X: u16 = AbsoluteAxisCode::ABS_X.0;
        const ABS_Y: u16 = AbsoluteAxisCode::ABS_Y.0;
        'a: loop {
            for ev in stylus.fetch_events().expect("Could not fetch events.") {
                match ev.event_type() {
                    EventType::ABSOLUTE => match ev.code() {
                        ABS_X => {
                            x = ev.value() - center.0;
                            moved(x, y);
                        },
                        ABS_Y => {
                            y = ev.value() - center.1;
                            moved(x, y);
                        },
                        _ => (), // Ignore other events
                    },
                    EventType::KEY => if ev.code() == KeyCode::BTN_STYLUS.code() {
                        println!("{:?}", active_sign);
                        break 'a;
                    },
                    _ => () // Ignore all other events
                }
            }
        }
        match signs.get(&active_sign) {
            None => println!("Invalid pattern."),
            Some(f) => f(),
        }
    };

    clos();
}

/// Returns the cell in wich the mouse is on based on its coordinates relative to the center of cell
/// 4.
///
/// Does not account for screen rotation.
fn get_cell_absolute(x: i32, y: i32, cell_size: u32) -> u8 {
    let half = cell_size as i32 / 2;
    match (x > half, x < -half, y > half, y < -half) {
        (false, true, false, true) => 0,
        (false, false, false, true) => 1,
        (true, false, false, true) => 2,
        (false, true, false, false) => 3,
        (false, false, false, false) => 4,
        (true, false, false, false) => 5,
        (false, true, true, false) => 6,
        (false, false, true, false) => 7,
        (true, false, true, false) => 8,
        _ => panic!("Impossible case.") // Mathematically impossible
    }
}

/// Same as [`get_cell_absolute`] but accounts for screen rotation.
/// The screen rotation is provided by [`start_reading_sings`] to fetch it only once.
fn get_cell_relative(x: i32, y: i32, cell_size: u32, rot: usize) -> u8 {
    // All rotation are given by counter-clockwise angles.
    const MAP: [[u8; 9]; 4] =
        [
            [0, 1, 2, 3, 4, 5, 6, 7, 8], // 0 -> 0° rotation
            [2, 5, 8, 1, 4, 7, 0, 3, 6], // 1 -> 90° rotation
            [8, 7, 6, 5, 4, 3, 2, 1, 0], // 2 -> 180° rotation
            [6, 3, 0, 7, 4, 1, 8, 5, 2], // 3 -> 270° rotation
        ];
    
    let cell = get_cell_absolute(x, y, cell_size);

    MAP[rot][cell as usize]
}
