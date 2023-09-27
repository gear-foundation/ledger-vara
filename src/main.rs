#![no_std]
#![no_main]

mod menu;

#[cfg(host_os = "macos")]
mod macos_lib;

use crate::menu::MainMenu;

use nanos_sdk::{
    buttons::ButtonEvent,
    io::{ApduHeader, Comm, Event, Reply, StatusWords},
};
use nanos_ui::ui::SingleMessage;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

#[no_mangle]
extern "C" fn sample_pending() {
    let mut comm = Comm::new();

    loop {
        SingleMessage::new("Pending").show();
        if let Event::Button(ButtonEvent::RightButtonRelease) = comm.next_event::<Ins>() {
            break;
        }
    }
    loop {
        SingleMessage::new("Ledger review").show();
        if let Event::Button(ButtonEvent::BothButtonsRelease) = comm.next_event::<Ins>() {
            break;
        }
    }
}

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = Comm::new();

    let mut main_menu = MainMenu::AppReady;
    main_menu.show();

    loop {
        match comm.next_event() {
            Event::Button(button) => main_menu.handle_button(button),
            Event::Command(ins) => {
                match handle_apdu(&mut comm, ins) {
                    Ok(()) => comm.reply_ok(),
                    Err(sw) => comm.reply(sw),
                }
                main_menu.show();
            }
            _ => (),
        }
    }
}

#[repr(u8)]
enum Ins {
    GetPubkey = 2,
    Sign,
    Menu,
    Exit = 0xFF,
}

impl From<ApduHeader> for Ins {
    fn from(header: ApduHeader) -> Ins {
        match header.ins {
            2 => Ins::GetPubkey,
            3 => Ins::Sign,
            4 => Ins::Menu,
            0xFF => Ins::Exit,
            _ => unreachable!("Invalid INS"),
        }
    }
}

fn handle_apdu(comm: &mut Comm, _ins: Ins) -> Result<(), Reply> {
    if comm.rx == 0 {
        return Err(StatusWords::NothingReceived.into());
    }

    Ok(())
}
