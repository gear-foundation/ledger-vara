mod page;

use ledger_device_sdk::buttons::ButtonEvent;
use ledger_device_sdk_ui::{
    bagls::{LEFT_S_ARROW, RIGHT_S_ARROW},
    layout::Draw,
    ui as gadgets,
};

pub use self::page::MenuPage;

pub trait Menu {
    fn page(&self) -> MenuPage;
    fn prev(&mut self);
    fn next(&mut self);
    fn action(&mut self) -> MenuAction;

    fn hide(&self) {
        self.page().hide();
    }

    fn show(&self) {
        gadgets::clear_screen();
        self.page().show();
    }

    fn handle_button_event(&mut self, button: ButtonEvent) -> MenuAction {
        match button {
            ButtonEvent::LeftButtonPress => {
                LEFT_S_ARROW.instant_display();
                MenuAction::Nothing
            }
            ButtonEvent::LeftButtonRelease => {
                LEFT_S_ARROW.instant_erase();
                self.prev();
                MenuAction::Nothing
            }
            ButtonEvent::RightButtonPress => {
                RIGHT_S_ARROW.instant_display();
                MenuAction::Nothing
            }
            ButtonEvent::RightButtonRelease => {
                RIGHT_S_ARROW.instant_erase();
                self.next();
                MenuAction::Nothing
            }
            ButtonEvent::BothButtonsPress => {
                LEFT_S_ARROW.instant_display();
                RIGHT_S_ARROW.instant_display();
                MenuAction::Nothing
            }
            ButtonEvent::BothButtonsRelease => {
                LEFT_S_ARROW.instant_erase();
                RIGHT_S_ARROW.instant_erase();
                self.action()
            }
        }
    }
}

pub enum MenuAction {
    Nothing,
    Update,
    Exit,
    Accept,
    Decline,
}
