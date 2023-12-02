use crate::menu::{Menu, MenuAction, MenuPage};
use core::str;
use ledger_device_sdk::{buttons::ButtonsState, ui::gadgets};

/// Get public key in interactive mode.
pub struct GetPublicKey {
    /// Menu.
    menu: GetPublicKeyMenu,
    /// Public key in 0x... hex format.
    pub key_hex: [u8; 66],
}

impl Default for GetPublicKey {
    fn default() -> Self {
        Self {
            menu: Default::default(),
            key_hex: [0; 66],
        }
    }
}

#[derive(Default)]
pub enum GetPublicKeyMenu {
    #[default]
    Review,
    PublicKey,
    Accept,
    Decline,
}

impl Menu for GetPublicKey {
    fn prev(&mut self) {
        self.page().hide();
        match self.menu {
            GetPublicKeyMenu::Review => self.menu = GetPublicKeyMenu::Decline,
            GetPublicKeyMenu::PublicKey => self.menu = GetPublicKeyMenu::Review,
            GetPublicKeyMenu::Accept => self.menu = GetPublicKeyMenu::PublicKey,
            GetPublicKeyMenu::Decline => self.menu = GetPublicKeyMenu::Accept,
        }
        self.page().show();
    }

    fn next(&mut self) {
        self.page().hide();
        match self.menu {
            GetPublicKeyMenu::Review => self.menu = GetPublicKeyMenu::PublicKey,
            GetPublicKeyMenu::PublicKey => self.menu = GetPublicKeyMenu::Accept,
            GetPublicKeyMenu::Accept => self.menu = GetPublicKeyMenu::Decline,
            GetPublicKeyMenu::Decline => self.menu = GetPublicKeyMenu::Review,
        }
        self.page().show();
    }

    fn action(&mut self) -> MenuAction {
        match self.menu {
            GetPublicKeyMenu::Accept => MenuAction::Accept,
            GetPublicKeyMenu::Decline => {
                self.menu = Default::default();
                MenuAction::Decline
            }
            _ => MenuAction::Nothing,
        }
    }

    fn page(&self) -> MenuPage {
        match self.menu {
            GetPublicKeyMenu::Review => MenuPage::new()
                .review_icon()
                .bold_text("Review")
                .text("Public key"),
            GetPublicKeyMenu::PublicKey => MenuPage::new()
                .text(str::from_utf8(&self.key_hex[..18]).unwrap())
                .text(str::from_utf8(&self.key_hex[19..34]).unwrap())
                .text(str::from_utf8(&self.key_hex[35..50]).unwrap())
                .text(str::from_utf8(&self.key_hex[51..]).unwrap()),
            GetPublicKeyMenu::Accept => MenuPage::new().accept_icon().bold_text("Accept"),
            GetPublicKeyMenu::Decline => MenuPage::new().decline_icon().bold_text("Decline"),
        }
    }
}

impl GetPublicKey {
    pub fn new(key: &[u8]) -> Self {
        let mut key_hex = [0; 66];
        key_hex[0] = b'0';
        key_hex[1] = b'x';
        _ = hex::encode_to_slice(key, &mut key_hex[2..]);
        Self {
            menu: Default::default(),
            key_hex,
        }
    }

    pub fn exec(&mut self) -> MenuAction {
        self.menu = Default::default();
        self.show();
        let mut state = ButtonsState::new();
        loop {
            if let Some(button) = gadgets::get_event(&mut state) {
                let action = self.handle_button_event(button);
                if matches!(action, MenuAction::Accept | MenuAction::Decline) {
                    return action;
                }
            }
        }
    }
}
