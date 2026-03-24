use std::collections::{HashMap};

use hyprland::keyword::{OptionValue, Keyword};

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
pub fn gen_signs() -> HashMap<u128, Box<dyn Fn()>> {
    let mut signs = std::collections::HashMap::<u128, Box<dyn Fn()>>::new();

    signs.insert(4125, Box::new(disable_touchscreen));

    signs
}

/// Toggle the disabling of the touchscreen.
pub fn disable_touchscreen() {
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
}
