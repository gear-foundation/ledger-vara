#![no_std]
#![no_main]

mod app;
mod error;
mod get_public_key;
mod menu;
mod settings;
mod signer;
mod transcript;

#[cfg(host_os = "macos")]
mod macos_lib;

use crate::{app::App, menu::Menu};
use ledger_device_sdk::io::{Comm, Event};

ledger_device_sdk::set_panic!(ledger_device_sdk::exiting_panic);

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
