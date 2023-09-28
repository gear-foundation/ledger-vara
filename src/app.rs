use crate::{
    apdu::{self, Ins},
    settings::Settings,
};
use include_gif::include_gif;
use ledger_prompts_ui::{
    Menu, MenuLabelBottom, MenuLabelTop, DASHBOARD_ICON, MENU_ICON_X, MENU_ICON_Y, SETTINGS_ICON,
};
use nanos_sdk::{
    buttons::ButtonEvent,
    io::{Comm, StatusWords},
};
use nanos_ui::{bagls::Icon, bitmaps::Glyph};

const APP_ICON_GLYPH: Glyph = Glyph::from_include(include_gif!("assets/icon_x.gif"));
const APP_ICON: Icon = Icon::from(&APP_ICON_GLYPH)
    .set_x(MENU_ICON_X)
    .set_y(MENU_ICON_Y);

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
    type BothResult = ();
    fn move_left(&mut self) {
        match self.menu {
            MainMenu::AppReady => self.menu = MainMenu::Quit,
            MainMenu::Version => self.menu = MainMenu::AppReady,
            MainMenu::Settings(true) => self.settings.move_left(),
            MainMenu::Settings(false) => self.menu = MainMenu::Version,
            MainMenu::Quit => self.menu = MainMenu::Settings(false),
        }
    }

    fn move_right(&mut self) {
        match self.menu {
            MainMenu::AppReady => self.menu = MainMenu::Version,
            MainMenu::Version => self.menu = MainMenu::Settings(false),
            MainMenu::Settings(true) => self.settings.move_right(),
            MainMenu::Settings(false) => self.menu = MainMenu::Quit,
            MainMenu::Quit => self.menu = MainMenu::AppReady,
        }
    }

    fn handle_both(&mut self) -> Option<Self::BothResult> {
        match self.menu {
            MainMenu::Settings(settings) => {
                if settings {
                    self.settings
                        .handle_both()
                        .map(|_| self.menu = MainMenu::Settings(false));
                } else {
                    self.menu = MainMenu::Settings(true);
                }
                None
            }
            MainMenu::Quit => Some(()),
            _ => None,
        }
    }

    fn label<'a>(&self) -> (MenuLabelTop<'a>, MenuLabelBottom<'a>) {
        match self.menu {
            MainMenu::AppReady => (
                MenuLabelTop::Icon(&APP_ICON),
                MenuLabelBottom {
                    text: "Vara app is ready",
                    bold: false,
                },
            ),
            MainMenu::Version => (
                MenuLabelTop::Text("Version"),
                MenuLabelBottom {
                    text: "1.0.0",
                    bold: false,
                },
            ),
            MainMenu::Settings(true) => self.settings.label(),
            MainMenu::Settings(false) => (
                MenuLabelTop::Icon(&SETTINGS_ICON),
                MenuLabelBottom {
                    text: "Settings",
                    bold: true,
                },
            ),
            MainMenu::Quit => (
                MenuLabelTop::Icon(&DASHBOARD_ICON),
                MenuLabelBottom {
                    text: "Quit",
                    bold: true,
                },
            ),
        }
    }
}

impl App {
    /// Show menu.
    pub fn show_menu(&self) {
        ledger_prompts_ui::show_menu(self);
    }

    /// Handle button event.
    pub fn handle_button(&mut self, button: ButtonEvent) {
        let update = matches!(
            button,
            ButtonEvent::LeftButtonRelease
                | ButtonEvent::RightButtonRelease
                | ButtonEvent::BothButtonsRelease
        );
        let exit = ledger_prompts_ui::handle_menu_button_event(self, button);
        if update && exit.is_none() {
            self.show_menu();
        }
        if exit.is_some() {
            nanos_sdk::exit_app(0)
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
