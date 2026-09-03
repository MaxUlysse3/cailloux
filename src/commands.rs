use std::collections::{HashMap}; use std::process::{Command};

use crate::tablet_handler::{get_stylus};
use crate::Env;

use evdev::{KeyCode, EventType};

use uinput::{Device as UDevice, device::Builder};

use hyprland::shared::{HyprData};
use hyprland::data::{Clients, Client};
use hyprland::keyword::{OptionValue, Keyword};
use hyprland::dispatch::{Dispatch, DispatchType, WindowIdentifier, FullscreenType};

/// Generate the `signs` dictionnary.
///
/// Inputs are read according to this table
/// ```
/// 0 | 1 | 2
/// 3 | 4 | 5
/// 6 | 7 | 8
/// ```
/// 
/// For exemple, the code `4125` refer to the input
/// ```
/// center -> north -> north-east -> east
/// ```
///
/// Because the grid is centered under the mouse on activation,
/// the sings should _always_ begin with 4.
/// Also, diagonal movement (`37` for exemple) cannot be detected.
pub fn gen_signs() -> HashMap<u128, Box<dyn Fn(&mut Env)>> {
    let mut signs = std::collections::HashMap::<u128, Box<dyn Fn(&mut Env)>>::new();

    signs.insert(4125, Box::new(disable_touchscreen));
    signs.insert(41012, Box::new(close_active));
    signs.insert(430125, Box::new(toggle_fullscreen));
    signs.insert(45214, Box::new(take_screenshot));
    // signs.insert(43, Box::new(emulate_click));

    signs
}

/// Toggle the disabling of the touchscreen.
pub fn disable_touchscreen(env: &mut Env) {
    // let name = "input:touchdevice:enabled";
    // let opt = Keyword::get(name);
    let new_val = format!("{}", if env.get_active_state() { 0 } else { 1 });
    println!("Toggling touchscreen");
    Command::new("hyprctl")
        .args(["eval", &("hl.config( {input = { touchdevice = { enabled = ".to_string() + &new_val.to_string() + " }}})")])
        .output()
        .expect("Could not change touchscreen activation.");
    let new_val = env.get_active_state();
    env.set_active_state(!new_val);
}

/// Close the active window.
pub fn close_active(_: &mut Env) {
    let active: Client = Clients::get().expect("Could not fetch clients data.").into_iter().filter(|w| (*w).focus_history_id == 0).collect::<Vec<_>>().pop().expect("Multiple active windows.");
    Dispatch::call(DispatchType::CloseWindow(WindowIdentifier::ProcessId(active.pid as u32))).expect("Could not close active window.");
}

/// Toggle the fullscreen state of the active window.
pub fn toggle_fullscreen(_: &mut Env) {
    Dispatch::call(DispatchType::ToggleFullscreen(FullscreenType::Real)).expect("Could not toggle fullscreen.");
}

/// Take a screenshot. Requires `flameshot`.
pub fn take_screenshot(_: &mut Env) {
    Command::new("flameshot")
        .arg("gui")
        .output()
        .expect("Could not launch flameshot.");
}

pub fn emulate_click(_: &mut Env) {
    let clos = move || {
        let mut stylus = get_stylus();
        // let mut vstylus = Builder::open(stylus.physical_path().unwrap()).expect("Could not open stylus.")
        //     .name("virtual stylus").unwrap()
        //     .create().expect("Could no initialize virtual input.");



        stylus.grab().expect("Could not grab stylus.");

        'a: loop {
            let mut out = vec![];

            for ev in stylus.fetch_events().expect("Could not fetch events.") {
                match ev.event_type() {
                    EventType::KEY => if ev.code() == KeyCode::BTN_TOUCH.code() {
                        println!("{:?}", ev);
                        break 'a;
                    },
                    _ => {
                        // Passthrough all other events
                        
                        out.push(ev);
                    },
                }
            }
            stylus.send_events(&out).expect("Coud not send events to stylus.");
            println!("{:?}", out);
        }

        stylus.ungrab().expect("Could not ungrab stylus.");
    };

    clos();
}
