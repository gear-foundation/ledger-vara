use ledger_prompts_ui::{Menu, MenuLabelBottom, MenuLabelTop, BACK_ICON};

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
    type BothResult = ();
    fn move_left(&mut self) {
        match self.menu {
            SettingsMenu::DebugMode => self.menu = SettingsMenu::Back,
            SettingsMenu::Back => self.menu = SettingsMenu::DebugMode,
        }
    }

    fn move_right(&mut self) {
        match self.menu {
            SettingsMenu::DebugMode => self.menu = SettingsMenu::Back,
            SettingsMenu::Back => self.menu = SettingsMenu::DebugMode,
        }
    }

    fn handle_both(&mut self) -> Option<Self::BothResult> {
        match self.menu {
            SettingsMenu::DebugMode => {
                self.debug_mode = !self.debug_mode;
                None
            }
            SettingsMenu::Back => {
                self.menu = Default::default();
                Some(())
            }
        }
    }

    fn label<'a>(&self) -> (MenuLabelTop<'a>, MenuLabelBottom<'a>) {
        match self.menu {
            SettingsMenu::DebugMode => (
                MenuLabelTop::Text("Debug mode"),
                MenuLabelBottom {
                    text: if self.debug_mode {
                        "Enabled"
                    } else {
                        "Disabled"
                    },
                    bold: false,
                },
            ),
            SettingsMenu::Back => (
                MenuLabelTop::Icon(&BACK_ICON),
                MenuLabelBottom {
                    text: "Back",
                    bold: false,
                },
            ),
        }
    }
}
