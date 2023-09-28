use crate::{
    apdu::{self, Ins},
    menu::{Menu, MenuAction, MenuPage},
    settings::Settings,
};
use nanos_sdk::{
    buttons::ButtonEvent,
    io::{Comm, StatusWords},
};

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
            //MenuAction::Update => self.show(),
            MenuAction::Exit => nanos_sdk::exit_app(0),
            _ => (),
        }
    }

    /// Handle command event.
    pub fn handle_command(&mut self, comm: &mut Comm, ins: Ins) {
        if comm.rx == 0 {
            comm.reply(StatusWords::NothingReceived);
            return;
        }
        _ = apdu::handle_apdu(ins)
            .map(|_| comm.reply_ok())
            .map_err(|err| comm.reply(err));
    }
}
