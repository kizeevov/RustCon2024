use iced::theme::{palette, Palette};
use iced::Color;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Dark,
}

impl Theme {
    pub fn palette(&self) -> Palette {
        match self {
            Self::Dark => DARK,
        }
    }

    pub fn extended_palette(&self) -> palette::Extended {
        match self {
            Theme::Dark => *EXTENDED_DARK,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}

pub const DARK: Palette = Palette {
    background: Color::from_rgb(
        0x20 as f32 / 255.0,
        0x22 as f32 / 255.0,
        0x25 as f32 / 255.0,
    ),
    text: Color::from_rgb(0.90, 0.90, 0.90),
    primary: Color::from_rgb(
        0x5E as f32 / 255.0,
        0x7C as f32 / 255.0,
        0xE2 as f32 / 255.0,
    ),
    success: Color::from_rgb(
        0x12 as f32 / 255.0,
        0x66 as f32 / 255.0,
        0x4F as f32 / 255.0,
    ),
    danger: Color::from_rgb(
        0xC3 as f32 / 255.0,
        0x42 as f32 / 255.0,
        0x3F as f32 / 255.0,
    ),
};

pub static EXTENDED_DARK: Lazy<palette::Extended> = Lazy::new(|| palette::Extended {
    background: palette::Background {
        base: palette::Pair {
            color: Color::BLACK,
            text: Default::default(),
        },
        weak: palette::Pair {
            color: Color::from_rgb(
                0x1C as f32 / 255.0,
                0x1C as f32 / 255.0,
                0x1E as f32 / 255.0,
            ),
            text: Default::default(),
        },
        strong: palette::Pair {
            color: Color::from_rgb(
                0x44 as f32 / 255.0,
                0x44 as f32 / 255.0,
                0x46 as f32 / 255.0,
            ),
            text: Default::default(),
        },
    },
    secondary: palette::Secondary {
        base: palette::Pair {
            color: Color::from_rgb(
                0xFF as f32 / 255.0,
                0x45 as f32 / 255.0,
                0x3A as f32 / 255.0,
            ),
            text: Color::WHITE,
        },
        weak: palette::Pair {
            color: Default::default(),
            text: Default::default(),
        },
        strong: palette::Pair {
            color: Default::default(),
            text: Default::default(),
        },
    },
    ..palette::Extended::generate(DARK)
});