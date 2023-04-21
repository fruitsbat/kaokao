use clap::{self, Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Clone, Parser, Debug, Default)]
pub struct Config {
    #[arg(long, default_value = "rofi")]
    pub rofi_binary: String,

    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    pub files: Vec<String>,

    /// show all unicode emoji
    #[arg(long)]
    pub disable_unicode: bool,

    /// show all builtin kaomoji
    #[arg(long)]
    pub disable_kaomoji: bool,

    /// show all builtin kaomoji
    #[arg(long)]
    pub disable_recent: bool,

    #[arg(short, long, default_value = "(*･ω･)✎ ")]
    pub prompt: String,

    #[arg(long)]
    pub show_duplicates: bool,
}

pub fn load_config() -> Config {
    Config::parse()
}

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum SkinTone {
    Default,
    Light,
    MediumLight,
    Medium,
    MediumDark,
    Dark,
    LightAndMediumLight,
    LightAndMedium,
    LightAndMediumDark,
    LightAndDark,
    MediumLightAndLight,
    MediumLightAndMedium,
    MediumLightAndMediumDark,
    MediumLightAndDark,
    MediumAndLight,
    MediumAndMediumLight,
    MediumAndMediumDark,
    MediumAndDark,
    MediumDarkAndLight,
    MediumDarkAndMediumLight,
    MediumDarkAndMedium,
    MediumDarkAndDark,
    DarkAndLight,
    DarkAndMediumLight,
    DarkAndMedium,
    DarkAndMediumDark,
}

impl From<&SkinTone> for emojis::SkinTone {
    fn from(item: &SkinTone) -> Self {
        match item {
            SkinTone::Default => Self::Default,
            SkinTone::Light => Self::Light,
            SkinTone::MediumLight => Self::MediumLight,
            SkinTone::Medium => Self::Medium,
            SkinTone::MediumDark => Self::MediumDark,
            SkinTone::Dark => Self::Dark,
            SkinTone::LightAndMediumLight => Self::DarkAndMedium,
            SkinTone::LightAndMedium => Self::DarkAndMedium,
            SkinTone::LightAndMediumDark => Self::LightAndMediumDark,
            SkinTone::LightAndDark => Self::LightAndDark,
            SkinTone::MediumLightAndLight => Self::MediumLightAndLight,
            SkinTone::MediumLightAndMedium => Self::MediumLightAndMedium,
            SkinTone::MediumLightAndMediumDark => Self::MediumLightAndMediumDark,
            SkinTone::MediumLightAndDark => Self::MediumLightAndDark,
            SkinTone::MediumAndLight => Self::MediumAndLight,
            SkinTone::MediumAndMediumLight => Self::MediumAndMediumLight,
            SkinTone::MediumAndMediumDark => Self::MediumAndMediumDark,
            SkinTone::MediumAndDark => Self::MediumAndDark,
            SkinTone::MediumDarkAndLight => Self::MediumDarkAndLight,
            SkinTone::MediumDarkAndMediumLight => Self::MediumDarkAndMediumLight,
            SkinTone::MediumDarkAndMedium => Self::MediumDarkAndMedium,
            SkinTone::MediumDarkAndDark => Self::MediumDarkAndDark,
            SkinTone::DarkAndLight => Self::DarkAndLight,
            SkinTone::DarkAndMediumLight => Self::DarkAndMediumLight,
            SkinTone::DarkAndMedium => Self::DarkAndMedium,
            SkinTone::DarkAndMediumDark => Self::DarkAndMediumDark,
        }
    }
}

/// this is important because this way the compiler will warn abt changes
impl From<&emojis::SkinTone> for SkinTone {
    fn from(item: &emojis::SkinTone) -> Self {
        match item {
            emojis::SkinTone::Default => Self::Default,
            emojis::SkinTone::Light => Self::Light,
            emojis::SkinTone::MediumLight => Self::MediumLight,
            emojis::SkinTone::Medium => Self::Medium,
            emojis::SkinTone::MediumDark => Self::MediumDark,
            emojis::SkinTone::Dark => Self::Dark,
            emojis::SkinTone::LightAndMediumLight => Self::DarkAndMedium,
            emojis::SkinTone::LightAndMedium => Self::DarkAndMedium,
            emojis::SkinTone::LightAndMediumDark => Self::LightAndMediumDark,
            emojis::SkinTone::LightAndDark => Self::LightAndDark,
            emojis::SkinTone::MediumLightAndLight => Self::MediumLightAndLight,
            emojis::SkinTone::MediumLightAndMedium => Self::MediumLightAndMedium,
            emojis::SkinTone::MediumLightAndMediumDark => Self::MediumLightAndMediumDark,
            emojis::SkinTone::MediumLightAndDark => Self::MediumLightAndDark,
            emojis::SkinTone::MediumAndLight => Self::MediumAndLight,
            emojis::SkinTone::MediumAndMediumLight => Self::MediumAndMediumLight,
            emojis::SkinTone::MediumAndMediumDark => Self::MediumAndMediumDark,
            emojis::SkinTone::MediumAndDark => Self::MediumAndDark,
            emojis::SkinTone::MediumDarkAndLight => Self::MediumDarkAndLight,
            emojis::SkinTone::MediumDarkAndMediumLight => Self::MediumDarkAndMediumLight,
            emojis::SkinTone::MediumDarkAndMedium => Self::MediumDarkAndMedium,
            emojis::SkinTone::MediumDarkAndDark => Self::MediumDarkAndDark,
            emojis::SkinTone::DarkAndLight => Self::DarkAndLight,
            emojis::SkinTone::DarkAndMediumLight => Self::DarkAndMediumLight,
            emojis::SkinTone::DarkAndMedium => Self::DarkAndMedium,
            emojis::SkinTone::DarkAndMediumDark => Self::DarkAndMediumDark,
            _ => Self::Default,
        }
    }
}
