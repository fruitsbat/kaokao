use crate::config::{Config, SkinTone};

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
    pub skintones: Option<Vec<SkinTone>>,
}

impl From<&Emoji> for Moji {
    fn from(emoji: &Emoji) -> Self {
        let skintones: Option<Vec<SkinTone>> = emoji.skin_tones().map(|skintones| {
            skintones
                // convert skin tone into native type
                .filter_map(|s| {
                    // hehe
                    s.skin_tone().map(|tone| SkinTone::from(&tone))
                })
                .collect::<Vec<SkinTone>>()
        });
        Moji {
            value: emoji.as_str().into(),
            description: description::get_description(emoji),
            skintones,
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
            skintones: None,
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
        assert_eq!(
            Moji::from(emojis::get("🤌").unwrap()),
            Moji {
                value: "🤌".into(),
                description: "pinched fingers, italian hand, chef's kiss".into(),
                skintones: Some(vec![
                    SkinTone::Default,
                    SkinTone::Light,
                    SkinTone::MediumLight,
                    SkinTone::Medium,
                    SkinTone::MediumDark,
                    SkinTone::Dark,
                ]),
            },
            "italian hand not parsed correctly"
        );
    }
}
