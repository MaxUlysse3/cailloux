use std::collections::{HashMap};
use std::thread;

use crate::tablet_handler::{get_stylus, Stylus};

use evdev::{Device, EventType, KeyCode, AbsoluteAxisCode};

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
            let cell = get_cell(x, y, cell_size);
            // println!("Cell : {cell:?} (last : {last_cell:?})");
            // println!("position : {x:?}, {y:?}");
            if cell != last_cell {
                println!("Cell : {cell:?} (last : {last_cell:?})");
                println!("position : {x:?}, {y:?}");
                counter += 1;
                println!("Detected a cell change (nb. {counter:?}), (active_sign: {active_sign:?})");
                active_sign = match active_sign.checked_mul(10) {
                    None => active_sign,
                    Some(val) => val + cell as u128,
                };
                last_cell = cell;
                println!("changed cell! {:?}", cell);
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

/// Return the cell in wich the mouse is on based on its coordinates relative to the center of cell
/// 4.
fn get_cell(x: i32, y: i32, cell_size: u32) -> u8 {
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

pub struct SignReader {
    cell_size: i32,
    last_cell: u8,
    active_sign: u32,
    signs: HashMap<u32, Box<dyn Fn()>>,
}

impl SignReader {
    // pub fn new(cell_size: i32) -> Self {
        
    // }
}
