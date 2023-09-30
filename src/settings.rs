use crate::menu::{Menu, MenuAction, MenuPage};

/// Settings.
#[derive(Default)]
pub struct Settings {
    /// Settings menu.
    menu: SettingsMenu,
    /// Debug mode.
    pub debug_mode: bool,
}

/// Settings menu.
#[derive(Default)]
pub enum SettingsMenu {
    /// Debug mode.
    #[default]
    DebugMode,
    /// Back to main menu.
    Back,
}

impl Menu for Settings {
    fn prev(&mut self) {
        self.page().hide();
        match self.menu {
            SettingsMenu::DebugMode => self.menu = SettingsMenu::Back,
            SettingsMenu::Back => self.menu = SettingsMenu::DebugMode,
        }
        self.page().show();
    }

    fn next(&mut self) {
        self.page().hide();
        match self.menu {
            SettingsMenu::DebugMode => self.menu = SettingsMenu::Back,
            SettingsMenu::Back => self.menu = SettingsMenu::DebugMode,
        }
        self.page().show();
    }

    fn action(&mut self) -> MenuAction {
        match self.menu {
            SettingsMenu::DebugMode => {
                self.debug_mode = !self.debug_mode;
                MenuAction::Update
            }
            SettingsMenu::Back => {
                self.menu = Default::default();
                MenuAction::Exit
            }
        }
    }

    fn page(&self) -> MenuPage {
        match self.menu {
            SettingsMenu::DebugMode => {
                MenuPage::new()
                    .bold_text("Debug mode")
                    .text(if self.debug_mode {
                        "Enabled"
                    } else {
                        "Disabled"
                    })
            }
            SettingsMenu::Back => MenuPage::new().back_icon().bold_text("Back"),
        }
    }
}
