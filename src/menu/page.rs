use include_gif::include_gif;
use ledger_device_sdk_ui::{
    bagls::{
        Icon, Label, CERTIFICATE_ICON, CHECKMARK_ICON, COGGLE_ICON, CROSS_ICON, DASHBOARD_X_ICON,
        LEFT_ARROW, RIGHT_ARROW,
    },
    bitmaps::{Glyph, BACK},
    layout::{Draw, Location},
    SCREEN_HEIGHT, SCREEN_WIDTH,
};

const SPACING: usize = 2;
const TEXT_HEIGHT: usize = 12;
const MIDDLE_TEXT_TOP_Y: usize = (SCREEN_HEIGHT - TEXT_HEIGHT) / 2;

const APP_GLYPH: Glyph = Glyph::from_include(include_gif!("assets/icon_x.gif"));

pub enum MenuPage<'a> {
    Empty,
    Single(PageItem<'a>),
    Double(PageItem<'a>, PageItem<'a>),
    Triple(PageItem<'a>, PageItem<'a>, PageItem<'a>),
    Quad(PageItem<'a>, PageItem<'a>, PageItem<'a>, PageItem<'a>),
}

pub enum PageItem<'a> {
    Icon(Icon<'a>),
    Text(Text<'a>),
}

pub struct Text<'a> {
    pub text: &'a str,
    pub bold: bool,
    pub y: usize,
}

impl<'a> MenuPage<'a> {
    pub const fn new() -> Self {
        MenuPage::Empty
    }

    pub const fn add(self, item: PageItem<'a>) -> Self {
        match self {
            MenuPage::Empty => MenuPage::Single(item),
            MenuPage::Single(first) => {
                let mut y = (SCREEN_HEIGHT - first.height() - item.height() - SPACING) / 2;
                let first = first.set_y(y);
                y += first.height() + SPACING;
                let item = item.set_y(y);
                MenuPage::Double(first, item)
            }
            MenuPage::Double(first, second) => {
                let mut y = (SCREEN_HEIGHT
                    - first.height()
                    - second.height()
                    - item.height()
                    - 2 * SPACING)
                    / 2;
                let first = first.set_y(y);
                y += first.height() + SPACING;
                let second = second.set_y(y);
                y += second.height() + SPACING;
                let item = item.set_y(y);
                MenuPage::Triple(first, second, item)
            }
            MenuPage::Triple(first, second, third) => {
                let mut y = (SCREEN_HEIGHT
                    - first.height()
                    - second.height()
                    - third.height()
                    - item.height()
                    - 3 * SPACING)
                    / 2;
                let first = first.set_y(y);
                y += first.height() + SPACING;
                let second = second.set_y(y);
                y += second.height() + SPACING;
                let third = third.set_y(y);
                y += third.height() + SPACING;
                let item = item.set_y(y);
                MenuPage::Quad(first, second, third, item)
            }
            MenuPage::Quad(..) => self,
        }
    }

    pub const fn text(self, text: &'a str) -> Self {
        let text = Text {
            text,
            bold: false,
            y: MIDDLE_TEXT_TOP_Y,
        };

        self.add(PageItem::Text(text))
    }

    pub const fn bold_text(self, text: &'a str) -> Self {
        let text = Text {
            text,
            bold: true,
            y: MIDDLE_TEXT_TOP_Y,
        };
        self.add(PageItem::Text(text))
    }

    pub const fn icon(self, icon: Icon<'a>) -> Self {
        let x = (SCREEN_WIDTH as u32 - icon.icon.width) / 2;
        let y = (SCREEN_HEIGHT as u32 - icon.icon.height) / 2;
        let icon = icon.set_x(x as i16).set_y(y as i16);
        self.add(PageItem::Icon(icon))
    }

    pub const fn app_icon(self) -> Self {
        self.icon(Icon::from(&APP_GLYPH))
    }

    pub const fn settings_icon(self) -> Self {
        self.icon(COGGLE_ICON)
    }

    pub const fn home_icon(self) -> Self {
        self.icon(DASHBOARD_X_ICON)
    }

    pub const fn back_icon(self) -> Self {
        self.icon(Icon::from(&BACK))
    }

    pub const fn review_icon(self) -> Self {
        self.icon(CERTIFICATE_ICON)
    }

    pub const fn accept_icon(self) -> Self {
        self.icon(CHECKMARK_ICON)
    }

    pub const fn decline_icon(self) -> Self {
        self.icon(CROSS_ICON)
    }

    pub fn hide(&self) {
        match self {
            MenuPage::Empty => (),
            MenuPage::Single(first) => first.instant_erase(),
            MenuPage::Double(first, second) => {
                first.instant_erase();
                second.instant_erase();
            }
            MenuPage::Triple(first, second, third) => {
                first.instant_erase();
                second.instant_erase();
                third.instant_erase();
            }
            MenuPage::Quad(first, second, third, fourth) => {
                first.instant_erase();
                second.instant_erase();
                third.instant_erase();
                fourth.instant_erase();
            }
        }
    }

    pub fn show(&self) {
        match self {
            MenuPage::Empty => (),
            MenuPage::Single(first) => first.instant_display(),
            MenuPage::Double(first, second) => {
                first.instant_display();
                second.instant_display();
            }
            MenuPage::Triple(first, second, third) => {
                first.instant_display();
                second.instant_display();
                third.instant_display();
            }
            MenuPage::Quad(first, second, third, fourth) => {
                first.instant_display();
                second.instant_display();
                third.instant_display();
                fourth.instant_display();
            }
        }
        LEFT_ARROW.instant_display();
        RIGHT_ARROW.instant_display();
    }
}

impl<'a> PageItem<'a> {
    pub const fn height(&self) -> usize {
        match self {
            PageItem::Icon(icon) => icon.icon.height as usize,
            PageItem::Text(_) => TEXT_HEIGHT,
        }
    }

    pub const fn set_y(self, y: usize) -> Self {
        match self {
            PageItem::Icon(mut icon) => {
                icon.pos.1 = y as i16;
                PageItem::Icon(icon)
            }
            PageItem::Text(mut text) => {
                text.y = y;
                PageItem::Text(text)
            }
        }
    }

    pub fn instant_display(&self) {
        match self {
            PageItem::Icon(icon) => icon.instant_display(),
            PageItem::Text(text) => text.to_label().instant_display(),
        }
    }

    pub fn instant_erase(&self) {
        match self {
            PageItem::Icon(icon) => icon.instant_erase(),
            PageItem::Text(text) => text.to_label().instant_erase(),
        }
    }
}

impl<'a> Text<'a> {
    pub const fn to_label(&self) -> Label<'a> {
        // TODO: Fix `from_const()` and `text()` methods in `mcu::Label`
        #[cfg(target_os = "nanos")]
        let mut label = Label::new().location(Location::Custom(self.y));

        #[cfg(not(target_os = "nanos"))]
        let mut label = Label::from_const(self.text).location(Location::Custom(self.y));

        label.text = self.text;

        if self.bold {
            label.bold()
        } else {
            label
        }
    }
}
