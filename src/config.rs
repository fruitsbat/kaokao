use clap::{self, Parser};

#[derive(Clone, Parser, Debug)]
pub struct Config {
    #[arg(long, default_value = "rofi")]
    pub rofi_binary: String,
}

pub fn load_config() -> Config {
    Config::parse()
}
