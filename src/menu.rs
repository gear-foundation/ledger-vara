use include_gif::include_gif;
use ledger_prompts_ui::{
    Menu, MenuLabelBottom, MenuLabelTop, BACK_ICON, DASHBOARD_ICON, MENU_ICON_X, MENU_ICON_Y,
    SETTINGS_ICON,
};
use nanos_sdk::buttons::ButtonEvent;
use nanos_ui::{bagls::Icon, bitmaps::Glyph};

const APP_ICON_GLYPH: Glyph = Glyph::from_include(include_gif!("assets/icon_x.gif"));
const APP_ICON: Icon = Icon::from(&APP_ICON_GLYPH)
    .set_x(MENU_ICON_X)
    .set_y(MENU_ICON_Y);

/// Main menu.
pub enum MainMenu {
    /// Application is ready.
    AppReady,
    /// Show version.
    Version,
    /// Setting submenu.
    Settings(Option<SettingsSubMenu>),
    /// Quit application.
    Quit,
}

/// Settings submenu.
pub enum SettingsSubMenu {
    /// Debug mode.
    DebugMode,
    /// Back to main menu.
    Back,
}

impl Menu for MainMenu {
    type BothResult = ();
    fn move_left(&mut self) {
        match self {
            MainMenu::AppReady => *self = MainMenu::Quit,
            MainMenu::Version => *self = MainMenu::AppReady,
            MainMenu::Settings(_) => *self = MainMenu::Version,
            MainMenu::Quit => *self = MainMenu::Settings(None),
        }
    }

    fn move_right(&mut self) {
        match self {
            MainMenu::AppReady => *self = MainMenu::Version,
            MainMenu::Version => *self = MainMenu::Settings(None),
            MainMenu::Settings(_) => *self = MainMenu::Quit,
            MainMenu::Quit => *self = MainMenu::AppReady,
        }
    }

    fn handle_both(&mut self) -> Option<Self::BothResult> {
        match self {
            MainMenu::Settings(submenu) => match submenu {
                Some(SettingsSubMenu::DebugMode) => {
                    *submenu = Some(SettingsSubMenu::Back);
                    None
                }
                Some(SettingsSubMenu::Back) => {
                    *submenu = None;
                    None
                }
                None => {
                    *submenu = Some(SettingsSubMenu::DebugMode);
                    None
                }
            },
            MainMenu::Quit => Some(()),
            _ => None,
        }
    }

    fn label<'a>(&self) -> (MenuLabelTop<'a>, MenuLabelBottom<'a>) {
        match self {
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
            MainMenu::Settings(submenu) => match submenu {
                Some(SettingsSubMenu::DebugMode) => (
                    MenuLabelTop::Text("Debug mode"),
                    MenuLabelBottom {
                        text: "Disabled",
                        bold: false,
                    },
                ),
                Some(SettingsSubMenu::Back) => (
                    MenuLabelTop::Icon(&BACK_ICON),
                    MenuLabelBottom {
                        text: "Back",
                        bold: false,
                    },
                ),
                None => (
                    MenuLabelTop::Icon(&SETTINGS_ICON),
                    MenuLabelBottom {
                        text: "Settings",
                        bold: true,
                    },
                ),
            },
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

impl MainMenu {
    /// Show menu.
    pub fn show(&self) {
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
            self.show();
        }
        if exit.is_some() {
            nanos_sdk::exit_app(0)
        }
    }
}
