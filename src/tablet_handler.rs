use std::io::Result;
use std::thread;
use std::sync::{Arc, Mutex, mpsc::{self, Sender, Receiver}};

use evdev::{Device, AbsoluteAxisCode, EventType, InputEvent};

#[derive(Debug)]
pub struct Stylus {
    device: Device,
}

#[derive(Copy, Clone)]
pub struct StylusData {
    pos_x: i32,
    pos_y: i32,
}

impl StylusData {
    pub fn new() -> Self {
        Self {
            pos_x: 0,
            pos_y: 0,
        }
    }

    pub fn get_pos_x(&self) -> i32 {
        self.pos_x
    }

    pub fn get_pos_y(&self) -> i32 {
        self.pos_y
    }
}

impl Stylus {
    pub fn init() -> Self {
        let mut styluses = evdev::enumerate().filter(|(_, dev)| dev.name().unwrap_or("").to_lowercase().contains("stylus")).collect::<Vec<_>>();
        if styluses.len() > 1 {
            panic!("Multiple tablets detected.");
        } else if styluses.len() == 0 {
            panic!("No styluses detected.");
        } else {
            Stylus {
                device: styluses.pop().unwrap().1,
            }
        }
    }

    pub fn get_pos(&self) -> (i32, i32) {
        let data = self.device.get_absinfo().unwrap().filter(|(ty, _)| *ty == AbsoluteAxisCode::ABS_X || *ty == AbsoluteAxisCode::ABS_Y).map(|(_, dat)| dat).collect::<Vec<_>>();

        let x = data[0].value();
        let y = data[1].value();

        (x, y)
    }

    pub fn listen_for(mut self, f: impl Fn(InputEvent, &StylusData) + Send + 'static) -> thread::JoinHandle<()> {
        let clos = move || {
            let mut dat = StylusData::new();
            loop {
                let evs = self.device.fetch_events().unwrap();
                for ev in evs {
                    if ev.event_type() == EventType::ABSOLUTE {
                        const ABS_X: u16 = AbsoluteAxisCode::ABS_X.0;
                        const ABS_Y: u16 = AbsoluteAxisCode::ABS_Y.0;
                        match ev.code() {
                            ABS_X => dat.pos_x = ev.value(),
                            ABS_Y => dat.pos_y = ev.value(),
                            _ => ()
                        }
                    }
                    f(ev, &dat);
                }
            }
        };

        thread::spawn(clos)
    }
}
