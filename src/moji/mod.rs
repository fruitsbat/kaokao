use crate::config::Config;

use emojis::{self, Emoji};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use std::error::Error;

mod description;
mod file;
mod kaomoji;
pub mod recent;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Moji {
    pub value: String,
    pub description: String,
    #[serde(default = "variations_default")]
    pub variations: Vec<Moji>,
}

fn variations_default() -> Vec<Moji> {
    vec![]
}

impl From<&Emoji> for Moji {
    fn from(emoji: &Emoji) -> Self {
        Moji {
            value: emoji.as_str().into(),
            description: description::get_description(emoji),
            variations: {
                match emoji.skin_tones() {
                    None => vec![],
                    Some(m) => m
                        .map(|m| Moji {
                            value: m.as_str().into(),
                            description: description::get_description(m),
                            variations: vec![],
                        })
                        .collect_vec(),
                }
            },
        }
    }
}

pub fn moji_to_string(moji: &Vec<Moji>) -> String {
    let mut s = String::new();
    for m in moji {
        s.push_str(&format!(
            "{}{} {}",
            if s.is_empty() { "" } else { "\n" },
            m.value,
            m.description
        ));
    }
    s
}

pub fn get_moji_list(cfg: &Config) -> Result<Vec<Moji>, Box<dyn Error>> {
    let mut mojis = vec![];
    if !cfg.disable_recent {
        mojis.append(&mut recent::load()?);
    }
    if !cfg.disable_unicode {
        mojis.append(&mut get_unicode_emoji());
    }
    if !cfg.disable_kaomoji {
        mojis.append(&mut builtin_kaomoji())
    }
    mojis.append(&mut file::load_moji_from_files(cfg)?);

    Ok(if cfg.show_duplicates {
        mojis
    } else {
        mojis.into_iter().unique().collect::<Vec<Moji>>()
    })
}

fn get_unicode_emoji() -> Vec<Moji> {
    emojis::iter().map(|m| m.into()).collect::<Vec<Moji>>()
}

fn builtin_kaomoji() -> Vec<Moji> {
    kaomoji::KAOMOJI
        .iter()
        .map(|row| Moji {
            value: row.0.into(),
            description: row.1.into(),
            variations: vec![],
        })
        .collect::<Vec<Moji>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builtin() {
        let unicode = get_unicode_emoji();
        assert!(!unicode.is_empty(), "no unicode emoji found");
        let kaomoji = builtin_kaomoji();
        assert!(!kaomoji.is_empty(), "no builtin kaomoji found");
    }

    #[test]
    fn moji_list() {
        let config = Config {
            disable_unicode: false,
            disable_kaomoji: false,
            disable_recent: true,
            files: vec![],
            ..Default::default()
        };
        let list = get_moji_list(&config).unwrap();
        assert!(!list.is_empty());
    }

    #[test]
    fn no_mojis() {
        let config = Config {
            disable_unicode: true,
            disable_kaomoji: true,
            disable_recent: true,
            files: vec![],
            ..Default::default()
        };

        let list = get_moji_list(&config).unwrap();
        assert!(list.is_empty(), "this list should be empty!");
    }

    #[test]
    fn convert() {
        let moji = Moji::from(emojis::get("🫱").unwrap());
        assert_eq!(moji.value, String::from("🫱"));
        assert_eq!(moji.description, String::from("rightwards hand"));
        assert!(moji.variations.len() > 0);
    }
}
