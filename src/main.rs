pub mod tablet_handler;
pub mod sign_reader;

use std::thread;
use std::time::{Duration};
use std::sync::mpsc;

use tablet_handler::{Stylus, StylusData};


// use raylib::prelude::*;

use evdev::{Device, EventType, KeyCode};

fn main() {
    // let stylus = Stylus::init();
    // let hdl = stylus.listen_for(|ev, dat| {
    //     match ev.event_type() {
    //         EventType::KEY => {
    //             if ev.code() == KeyCode::BTN_STYLUS.code() && ev.value() == 1 {
    //                 println!("{:?}, {:?}", dat.get_pos_x(), dat.get_pos_y());
    //             }
    //         },
    //         _ => (),
    //     }
    // });

    // hdl.join().unwrap();
    
    let mut styluses = evdev::enumerate().filter(|(_, dev)| dev.name().unwrap_or("").to_lowercase().contains("stylus")).collect::<Vec<_>>();
    let mut s1 = if styluses.len() > 1 {
        panic!("Multiple tablets detected.");
    } else if styluses.len() == 0 {
        panic!("No styluses detected.");
    } else {
        styluses.pop().unwrap().1
    };

    let mut styluses = evdev::enumerate().filter(|(_, dev)| dev.name().unwrap_or("").to_lowercase().contains("stylus")).collect::<Vec<_>>();
    let mut s2 = if styluses.len() > 1 {
        panic!("Multiple tablets detected.");
    } else if styluses.len() == 0 {
        panic!("No styluses detected.");
    } else {
        styluses.pop().unwrap().1
    };


    let t = thread::spawn(move || {
        loop {
            for e in s2.fetch_events().expect("Error fetching events from thread 2.") {
                match e.event_type() {
                    EventType::KEY => {
                        if e.code() == KeyCode::BTN_STYLUS.code() && e.value() == 1 {
                            println!("Button pressed from thread 2!");
                        }
                    },
                    _ => (),
                }
            }
        }
    });

    loop {
        for e in s1.fetch_events().expect("Error fetching events from thread 1.") {
            match e.event_type() {
                EventType::KEY => {
                    if e.code() == KeyCode::BTN_STYLUS.code() && e.value() == 1 {
                        println!("Button pressed from thread 1!");
                    }
                },
                _ => (),
            }
        }
    }

    // t.join().unwrap();

    // let (mut rl, thread) = raylib::init()
    //     .size(640, 480)
    //     .title("Hello, World!")
    //     .transparent()
    //     .undecorated()
    //     .build();

    // rl.set_window_opacity(0.5);

    // while !rl.window_should_close() {
    //     let mut d = rl.begin_drawing(&thread);

    //     d.clear_background(Color::WHITE.alpha(0.5));
    //     d.draw_text("Hello, World!", 12, 12, 20, Color::BLACK.alpha(0.5));
    // }
}
