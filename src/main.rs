pub mod tablet_handler;

use std::thread;
use std::time::{Duration};
use std::sync::mpsc;

use tablet_handler::{Stylus, StylusData};

use evdev::{Device, EventType, KeyCode};

fn main() {
    // loop {
    //     let evs = stylus.fetch_events();
    //     for e in evs.unwrap() {
    //         match e.event_type() {
    //             EventType::KEY => println!("{:?}", e),
    //             EventType::ABSOLUTE => println!("{:?}", e),
    //             _ => (),
    //         }
    //     }
    // }

    
    let stylus = Stylus::init();
    let hdl = stylus.listen_for(|ev, dat| {
        match ev.event_type() {
            EventType::KEY => {
                if ev.code() == KeyCode::BTN_STYLUS.code() && ev.value() == 1 {
                    println!("{:?}, {:?}", dat.get_pos_x(), dat.get_pos_y());
                }
            },
            _ => (),
        }
    });

    hdl.join().unwrap();
}
