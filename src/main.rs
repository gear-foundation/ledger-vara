#![no_std]
#![no_main]

mod utils;

#[cfg(host_os = "macos")]
mod macos_lib;

use core::str;
use nanos_sdk::{
    buttons::ButtonEvent,
    ecc::{Secp256k1, SeedDerive},
    io::{ApduHeader, Comm, Event, Reply, StatusWords, SyscallError},
};
use nanos_ui::ui::{self, Menu, MessageScroller, SingleMessage, Validator};

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

pub const BIP32_PATH: [u32; 5] = nanos_sdk::ecc::make_bip32_path(b"m/44'/535348'/0'/0/0");

/// Display public key in two separate
/// message scrollers
fn show_pubkey() {
    let pubkey = Secp256k1::derive_from_path(&BIP32_PATH).public_key();
    match pubkey {
        Ok(pk) => {
            {
                let hex0 = utils::to_hex(&pk.as_ref()[1..33]).unwrap();
                let m = str::from_utf8(&hex0).unwrap();
                MessageScroller::new(m).event_loop();
            }
            {
                let hex1 = utils::to_hex(&pk.as_ref()[33..65]).unwrap();
                let m = str::from_utf8(&hex1).unwrap();
                MessageScroller::new(m).event_loop();
            }
        }
        Err(_) => ui::popup("Error"),
    }
}

/// Basic nested menu. Will be subject
/// to simplifications in the future.
fn menu_example() {
    loop {
        match Menu::new(&[&"PubKey", &"Infos", &"Back", &"Exit App"]).show() {
            0 => show_pubkey(),
            1 => loop {
                match Menu::new(&[&"Copyright", &"Authors", &"Back"]).show() {
                    0 => ui::popup("2023 Gear Foundation"),
                    1 => ui::popup("Gear Foundation"),
                    _ => break,
                }
            },
            2 => return,
            3 => nanos_sdk::exit_app(0),
            _ => unreachable!("Invalid menu index"),
        }
    }
}

/// This is the UI flow for signing, composed of a scroller
/// to read the incoming message, a panel that requests user
/// validation, and an exit message.
fn sign_ui(message: &[u8]) -> Result<Option<([u8; 72], u32, u32)>, SyscallError> {
    ui::popup("Message review");

    {
        let hex = utils::to_hex(message).map_err(|_| SyscallError::Overflow)?;
        let m = str::from_utf8(&hex).map_err(|_| SyscallError::InvalidParameter)?;

        MessageScroller::new(m).event_loop();
    }

    if Validator::new("Sign ?").ask() {
        let signature = Secp256k1::derive_from_path(&BIP32_PATH)
            .deterministic_sign(message)
            .map_err(|_| SyscallError::Unspecified)?;
        ui::popup("Done!");
        Ok(Some(signature))
    } else {
        ui::popup("Cancelled");
        Ok(None)
    }
}

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

    loop {
        // Draw some 'welcome' screen
        SingleMessage::new("W e l c o m e").show();

        // Wait for either a specific button push to exit the app
        // or an APDU command
        match comm.next_event() {
            Event::Button(ButtonEvent::RightButtonRelease) => nanos_sdk::exit_app(0),
            Event::Command(ins) => match handle_apdu(&mut comm, ins) {
                Ok(()) => comm.reply_ok(),
                Err(sw) => comm.reply(sw),
            },
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

fn handle_apdu(comm: &mut Comm, ins: Ins) -> Result<(), Reply> {
    if comm.rx == 0 {
        return Err(StatusWords::NothingReceived.into());
    }

    match ins {
        Ins::GetPubkey => {
            let pk = Secp256k1::derive_from_path(&BIP32_PATH)
                .public_key()
                .map_err(|x| Reply(0x6e_u16 | (x as u16 & 0xff)))?;
            comm.append(pk.as_ref());
        }
        Ins::Sign => {
            let out = sign_ui(comm.get_data()?)?;
            if let Some((signature_buf, length, _)) = out {
                comm.append(&signature_buf[..length as usize])
            }
        }
        Ins::Menu => menu_example(),
        Ins::Exit => nanos_sdk::exit_app(0),
    }
    Ok(())
}
