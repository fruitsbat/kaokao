use clap::{self, Parser, ValueEnum};

#[derive(Clone, Parser, Debug)]
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

    #[arg(long, value_enum, num_args = 1.., value_delimiter = ' ', default_value = "default dark medium-dark medium medium-light light")]
    pub skin_tones: Vec<SkinTone>,
}

pub fn load_config() -> Config {
    Config::parse()
}

#[derive(Clone, Debug, ValueEnum, PartialEq)]
pub enum SkinTone {
    Default,
    Light,
    MediumLight,
    Medium,
    MediumDark,
    Dark,
}

impl From<&SkinTone> for emojis::SkinTone {
    fn from(item: &SkinTone) -> Self {
        match item {
            SkinTone::Default => emojis::SkinTone::Default,
            SkinTone::Light => emojis::SkinTone::Light,
            SkinTone::MediumLight => emojis::SkinTone::MediumLight,
            SkinTone::Medium => emojis::SkinTone::Medium,
            SkinTone::MediumDark => emojis::SkinTone::MediumDark,
            SkinTone::Dark => emojis::SkinTone::Dark,
        }
    }
}

/// this is important because this way the compiler will warn abt changes
impl From<&emojis::SkinTone> for SkinTone {
    fn from(item: &emojis::SkinTone) -> Self {
        match item {
            emojis::SkinTone::Default => SkinTone::Default,
            emojis::SkinTone::Light => SkinTone::Light,
            emojis::SkinTone::MediumLight => SkinTone::MediumLight,
            emojis::SkinTone::Medium => SkinTone::Medium,
            emojis::SkinTone::MediumDark => SkinTone::MediumDark,
            emojis::SkinTone::Dark => SkinTone::Dark,
        }
    }
}
