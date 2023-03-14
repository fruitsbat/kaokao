use crate::config::Config;
use emojis;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{error::Error, fs};

#[derive(Serialize, Deserialize)]
pub struct Moji {
    pub value: String,
    pub description: String,
}

impl Moji {
    pub fn from_emoji(emoji: &emojis::Emoji) -> Self {
        Moji {
            value: emoji.as_str().into(),
            description: emoji.name().into(),
        }
    }
}

pub fn moji_to_string(moji: &Vec<Moji>) -> String {
    let mut s = String::new();
    for m in moji {
        s.push_str(&format!(
            "{}{} {}",
            if s.len() == 0 { "" } else { "\n" },
            m.value,
            m.description
        ));
    }
    s
}

pub fn get_moji_list(cfg: &Config) -> Result<Vec<Moji>, Box<dyn Error>> {
    let mut mojis = vec![];
    mojis.append(
        &mut emojis::iter()
            .map(|e| Moji::from_emoji(e))
            .collect::<Vec<Moji>>(),
    );
    mojis.append(&mut load_moji_from_files(&cfg)?);
    Ok(mojis)
}

fn load_moji_from_files(cfg: &Config) -> Result<Vec<Moji>, Box<dyn Error>> {
    let mut vec_moji = vec![];
    for p in &cfg.files {
        let filecontents = fs::read_to_string(p)?;
        let mut m: Vec<Moji> = serde_json::from_str(&filecontents)?;
        vec_moji.append(&mut m);
    }
    Ok(vec_moji)
}
