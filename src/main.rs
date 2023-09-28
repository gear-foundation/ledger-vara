#![no_std]
#![no_main]

mod apdu;
mod app;
mod settings;

#[cfg(host_os = "macos")]
mod macos_lib;

use crate::app::App;
use nanos_sdk::io::{Comm, Event};

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = Comm::new();

    let mut app: App = Default::default();
    app.show_menu();

    loop {
        match comm.next_event() {
            Event::Button(button) => app.handle_button(button),
            Event::Command(ins) => app.handle_command(&mut comm, ins),
            _ => (),
        }
    }
}
