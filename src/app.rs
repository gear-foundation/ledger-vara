use crate::{
    error::ErrorCode,
    menu::{Menu, MenuAction, MenuPage},
    settings::Settings,
    signer::Signer,
};
use nanos_sdk::{
    buttons::ButtonEvent,
    io::{ApduHeader, Comm},
};

const APDU_CLA: u8 = 0x89;

const INS_GET_VERSION: u8 = 0x00;
const INS_GET_PUBLIC_KEY: u8 = 0x01;
const INS_SIGN: u8 = 0x02;
const INS_QUIT: u8 = 0xFF;

const COMMAND_START: u8 = 0x00;
const COMMAND_APPEND: u8 = 0x01;
const COMMAND_FINALIZE: u8 = 0x02;

const MODE_INTERACTIVE: u8 = 0x01;

/// Application struct.
#[derive(Default)]
pub struct App {
    /// Main menu.
    pub menu: MainMenu,
    /// Settings.
    pub settings: Settings,
    /// Signer.
    pub signer: Signer,
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
    pub fn handle_command(&mut self, comm: &mut Comm, header: ApduHeader) -> Result<(), ErrorCode> {
        if comm.rx == 0 {
            return Err(ErrorCode::NothingReceived);
        }
        if header.cla != APDU_CLA {
            return Err(ErrorCode::BadCla);
        }
        match header.ins {
            INS_GET_VERSION => {
                let major: u16 = env!("CARGO_PKG_VERSION_MAJOR").parse()?;
                comm.append(&major.to_be_bytes());
                let minor: u16 = env!("CARGO_PKG_VERSION_MINOR").parse()?;
                comm.append(&minor.to_be_bytes());
                let patch: u16 = env!("CARGO_PKG_VERSION_PATCH").parse()?;
                comm.append(&patch.to_be_bytes());
            }
            INS_GET_PUBLIC_KEY => {
                self.signer.clear();
                self.signer.set_scheme(header.p2.try_into()?);
                self.signer.set_path(get_path(comm)?);
                let key = self.signer.get_public_key()?;
                if header.p1 == MODE_INTERACTIVE {
                    return Err(ErrorCode::Unimplemented);
                }
                comm.append(&key);
            }
            INS_SIGN => {
                let command = header.p1;
                match command {
                    COMMAND_START => {
                        self.signer.clear();
                        self.signer.set_scheme(header.p2.try_into()?);
                        self.signer.set_path(get_path(comm)?);
                    }
                    COMMAND_APPEND => {
                        let data_len = comm.apdu_buffer[4] as usize;
                        let data = &comm.apdu_buffer[5..5 + data_len];
                        self.signer.append_message(data)?;
                    }
                    COMMAND_FINALIZE => {
                        let signature = self.signer.sign()?;
                        self.signer.clear();
                        comm.append(&signature);
                    }
                    _ => return Err(ErrorCode::BadP1P2),
                }
            }
            INS_QUIT => nanos_sdk::exit_app(0),
            _ => return Err(ErrorCode::BadIns),
        }
        Ok(())
    }
}

fn get_path(comm: &Comm) -> Result<[u32; 5], ErrorCode> {
    let data_len = comm.apdu_buffer[4];
    if data_len != 20 {
        return Err(ErrorCode::BadLen);
    }
    let mut path = [0; 5];
    for i in 0..5 {
        let idx = 5 + i * 4;
        path[i] = u32::from_le_bytes(comm.apdu_buffer[idx..idx + 4].try_into()?);
    }
    Ok(path)
}
