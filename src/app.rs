use crate::{
    menu::{Menu, MenuAction, MenuPage},
    settings::Settings,
    utils,
};
use nanos_sdk::{
    buttons::ButtonEvent,
    io::{ApduHeader, Comm, StatusWords},
};

const APDU_CLA: u8 = 0x89;
const INS_GET_VERSION: u8 = 0x00;
const INS_GET_PUBLIC_KEY: u8 = 0x01;
const INS_QUIT: u8 = 0xFF;

/// Application struct.
#[derive(Default)]
pub struct App {
    /// Main menu.
    pub menu: MainMenu,
    /// Settings.
    pub settings: Settings,
}

/// Main menu.
#[derive(Default)]
pub enum MainMenu {
    /// Application is ready.
    #[default]
    AppReady,
    /// Show version.
    Version,
    /// Settings submenu.
    Settings(bool),
    /// Quit application.
    Quit,
}

impl Menu for App {
    fn prev(&mut self) {
        self.page().hide();
        match self.menu {
            MainMenu::AppReady => self.menu = MainMenu::Quit,
            MainMenu::Version => self.menu = MainMenu::AppReady,
            MainMenu::Settings(true) => {
                self.settings.prev();
            }
            MainMenu::Settings(false) => self.menu = MainMenu::Version,
            MainMenu::Quit => self.menu = MainMenu::Settings(false),
        }
        self.page().show();
    }

    fn next(&mut self) {
        self.page().hide();
        match self.menu {
            MainMenu::AppReady => self.menu = MainMenu::Version,
            MainMenu::Version => self.menu = MainMenu::Settings(false),
            MainMenu::Settings(true) => {
                self.settings.next();
            }
            MainMenu::Settings(false) => self.menu = MainMenu::Quit,
            MainMenu::Quit => self.menu = MainMenu::AppReady,
        }
        self.page().show();
    }

    fn action(&mut self) -> MenuAction {
        match self.menu {
            MainMenu::Settings(settings) => {
                self.page().hide();
                if settings {
                    let action = self.settings.action();
                    if let MenuAction::Exit = action {
                        self.menu = MainMenu::Settings(false);
                    }
                } else {
                    self.menu = MainMenu::Settings(true);
                }
                self.page().show();
                MenuAction::Update
            }
            MainMenu::Quit => MenuAction::Exit,
            _ => MenuAction::Nothing,
        }
    }

    fn page(&self) -> MenuPage {
        match self.menu {
            MainMenu::AppReady => MenuPage::new()
                .app_icon()
                .bold_text("Vara App")
                .text("Ready"),
            MainMenu::Version => MenuPage::new()
                .bold_text("Version")
                .text(env!("CARGO_PKG_VERSION")),
            MainMenu::Settings(true) => self.settings.page(),
            MainMenu::Settings(false) => MenuPage::new().settings_icon().bold_text("Settings"),
            MainMenu::Quit => MenuPage::new().home_icon().bold_text("Quit"),
        }
    }
}

impl App {
    /// Handle button event.
    pub fn handle_button(&mut self, button: ButtonEvent) {
        match self.handle_button_event(button) {
            MenuAction::Exit => nanos_sdk::exit_app(0),
            _ => (),
        }
    }

    /// Handle command event.
    pub fn handle_command(
        &mut self,
        comm: &mut Comm,
        header: ApduHeader,
    ) -> Result<(), StatusWords> {
        if comm.rx == 0 {
            return Err(StatusWords::NothingReceived);
        }
        if header.cla != APDU_CLA {
            return Err(StatusWords::BadCla);
        }
        match header.ins {
            INS_GET_VERSION => {
                let major = utils::bytes_to_u16(env!("CARGO_PKG_VERSION_MAJOR").as_bytes());
                comm.append(&major.to_be_bytes());
                let minor = utils::bytes_to_u16(env!("CARGO_PKG_VERSION_MINOR").as_bytes());
                comm.append(&minor.to_be_bytes());
                let patch = utils::bytes_to_u16(env!("CARGO_PKG_VERSION_PATCH").as_bytes());
                comm.append(&patch.to_be_bytes());
            }
            INS_GET_PUBLIC_KEY => {
                let key = utils::get_public_key();
                comm.append(&key);
            }
            INS_QUIT => nanos_sdk::exit_app(0),
            _ => return Err(StatusWords::BadIns),
        }
        Ok(())
    }
}
