use std::collections::{HashMap};
use std::thread;

use crate::tablet_handler::{get_stylus, Stylus};

use evedv::{Device, EventType, KeyCode}

pub fn start_reading_signs(center: (i32, i32), cell_size: u32, signs: &HashMap<u32, Box<dyn Fn()>>) {
    let mut stylus = get_stylus();;
    let mut active_sign = 0;
    let clos = move || {
        loop {
            for ev in stylus.fetchevents
        }
    }
}

fn get_cell(x: i32, y: i32, cell_size: u32) -> u8 {
    // TODO
}

pub struct SignReader {
    cell_size: i32,
    last_cell: u8,
    active_sign: u32,
    signs: HashMap<u32, Box<dyn Fn()>>,
}

impl SignReader {
    pub fn new(cell_size: i32) -> Self {
        
    }
}
