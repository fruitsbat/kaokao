use clap::{self, Parser};

#[derive(Clone, Parser, Debug)]
pub struct Config {
    #[arg(long, default_value = "rofi")]
    pub rofi_binary: String,

    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    pub files: Vec<String>,
}

pub fn load_config() -> Config {
    Config::parse()
}
