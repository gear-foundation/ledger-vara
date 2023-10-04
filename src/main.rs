#![no_std]
#![no_main]

mod app;
mod error;
mod menu;
mod settings;
mod utils;

#[cfg(host_os = "macos")]
mod macos_lib;

use crate::{app::App, menu::Menu};
use nanos_sdk::io::{Comm, Event};

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = Comm::new();

    let mut app: App = Default::default();
    app.show();

    loop {
        match comm.next_event() {
            Event::Button(button) => app.handle_button(button),
            Event::Command(header) => {
                _ = app
                    .handle_command(&mut comm, header)
                    .map(|_| comm.reply_ok())
                    .map_err(|err| comm.reply(err));
            }
            _ => (),
        }
    }
}
