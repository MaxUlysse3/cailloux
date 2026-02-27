use evdev::{Device, InputEvent};

fn main() {

    let mut device = Device::open("/dev/input/event17").expect("Could not open device.");

    loop {
        for event in device.fetch_events().unwrap() {
            println!("{:?}", event.event_type());
            println!("{:?}", event.code());
            println!("{:?}", event.value());
        }
    }
}
