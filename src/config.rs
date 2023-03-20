use clap::{self, Parser};

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
}

pub fn load_config() -> Config {
    Config::parse()
}
