use crate::config::{self, Config};
use csv;
use directories;
use emojis::{self, Emoji};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{error::Error, fs};

mod description;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Moji {
    pub value: String,
    pub description: String,
}

impl From<&Emoji> for Moji {
    fn from(emoji: &Emoji) -> Self {
        Moji {
            value: emoji.as_str().into(),
            description: description::get_description(emoji),
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
    if !cfg.disable_recent {
        mojis.append(&mut load_recent()?);
    }
    if !cfg.disable_unicode {
        mojis.append(&mut get_unicode_emoji(cfg)?);
    }
    if !cfg.disable_kaomoji {
        mojis.append(&mut builtin_kaomoji())
    }
    mojis.append(&mut load_moji_from_files(&cfg)?);

    Ok(if cfg.show_duplicates {
        mojis
    } else {
        mojis.into_iter().unique().collect::<Vec<Moji>>()
    })
}

fn get_unicode_emoji(cfg: &Config) -> Result<Vec<Moji>, Box<dyn Error>> {
    let mut mojis = vec![];
    for m in emojis::iter() {
        let m = match m.skin_tones() {
            Some(skin_tones) => skin_tones,
            None => {
                mojis.push(m.into());
                continue;
            }
        };
        let mut m = m
            .filter(|value| {
                cfg.skin_tones.contains({
                    let skin_tone = value.skin_tone();
                    &config::SkinTone::from(&skin_tone.unwrap())
                })
            })
            .map(|value| value.into())
            .collect::<Vec<Moji>>();
        mojis.append(&mut m)
    }

    mojis.append(&mut emojis::iter().map(|e| e.into()).collect::<Vec<Moji>>());
    Ok(mojis)
}

fn load_moji_from_files(cfg: &Config) -> Result<Vec<Moji>, Box<dyn Error>> {
    let mut vec_moji = vec![];
    for p in &cfg.files {
        let filecontents = fs::read_to_string(p)?;
        let mut m: Vec<Moji> = if p.ends_with(".json") {
            serde_json::from_str(&filecontents)?
        // default to csv
        } else {
            let mut mojis = vec![];
            let mut reader = csv::Reader::from_reader(filecontents.as_bytes());
            for moji in reader.deserialize() {
                let moji: Moji = moji?;
                mojis.push(moji);
            }
            mojis
        };
        vec_moji.append(&mut m);
    }
    Ok(vec_moji)
}

pub fn save_recent(moji: Moji) -> Result<(), Box<dyn Error>> {
    let mut recents = vec![moji.clone()];
    recents.append(
        &mut load_recent()?
            .iter()
            .filter(|&m| m.clone() != moji)
            .map(|m| m.clone())
            .collect::<Vec<Moji>>(),
    );
    let mut new_recents = vec![];
    let mut i = 0;
    for r in recents {
        // limit to 100 most recent
        if i >= 100 {
            break;
        }
        new_recents.push(r);
        i += 1;
    }
    let base_dirs = directories::BaseDirs::new().ok_or("failed to find base dir!")?;
    let data_dir = base_dirs.data_dir();
    fs::create_dir_all(data_dir.join("kaokao"))?;
    fs::write(
        data_dir.join("kaokao/recent.json"),
        serde_json::to_string(&new_recents)?.as_bytes(),
    )?;
    Ok(())
}

fn load_recent() -> Result<Vec<Moji>, Box<dyn Error>> {
    let base_dir = directories::BaseDirs::new().ok_or("could not load base dir")?;
    let path = base_dir.data_dir().join("kaokao/recent.json");
    if !path.exists() {
        return Ok(vec![]);
    }
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

fn builtin_kaomoji() -> Vec<Moji> {
    vec![
        ("(ﾉ≧∇≦)ﾉ ﾐ ┸━┸", "tableflip"),
        ("(* >ω<)=3ACHOOE!", "sneezing"),
        ("┐(°‿°)┌", "shrugging"),
        ("ᕙ(⇀‸↼‶)ᕗ", "flexing"),
        ("(❁ᴗ͈ ˬ ᴗ͈)ᶻᶻᶻ✧", "sleeping"),
        ("(:˒[￣] ᵍᵒᵒᵈ ⁿᶦᵍʰᵗ", "good night, sleeping"),
        ("≖‿≖", "skeptical, dead inside"),
        ("ಠ_ಠ", "disapproval"),
        (" /ᐠ｡ꞈ｡ᐟ\\", "cat, kitty"),
        ("─=≡Σ((( つ•̀ω•́)つLET’SGO!", "let's go"),
        ("( ˘▽˘)っ♨", "tea, coffee, hot drink, food"),
        ("(๑¯ิ̑﹃ ¯ิ̑๑)", "hungry"),
        ("(─‿‿─)", "skeptical, uwu"),
        ("(ू˃̣̣̣̣̣̣︿˂̣̣̣̣̣̣ ू)", "cry"),
        ("(✼ X̥̥̥ ‸ X̥̥̥)", "crying"),
        ("╥﹏╥", "crying"),
        ("（＾▽＾（＾▽＾＊）", "hugging, friends"),
        ("（⋆＾－＾⋆）", "smiling, blushing"),
        ("（＾－＾）", "smiling"),
        ("（＾³＾）～♪", "whistling"),
        ("(㇏(•̀ ᢍ •́ )ノ)", "bat"),
        (" ♡(ᐢ ᴥ ᐢし)", "dog, love"),
        ("૮ ˘ﻌ˘ ა", "dog, sleeping"),
        ("(๑•﹏•)⋆* ⁑⋆*", "confused"),
        ("(ﾉ>｡☆)ﾉ", "surprised"),
        ("(」ﾟﾛﾟ)｣NOOOooooo━", "surprised"),
        ("ᵒʰ(⑉・̆⌓・̆⑉)ɴᴏ", "nervous"),
        ("(*≧m≦*)", "angry"),
        ("(*･ω･)✎ ", "writing"),
        ("(๑˃ᴗ˂)ﻭ", "fight, cute, fist"),
        ("(⌒‿⌒)", "smile"),
        ("(─‿‿─)♡", "uwu, love"),
        ("(⌒_⌒;)", "sweat, emberassed"),
        ("(≧◡≦) ♡", "love, blushing"),
        ("(；⌣̀_⌣́)", "seething"),
        ("(」＞＜)」", "you hear about video games"),
        ("ヾ( ￣O￣)ツ", "yawn, sleepy, bored"),
        ("(＞﹏＜)", "sour"),
        ("(・`ω´・) ", "rage, angry"),
        ("(╬ Ò﹏Ó)", "angry, rage"),
        ("(T⌓T)", "trolled"),
        ("(×_×)", "dead"),
        ("(￢_￢)", "sideeyes"),
        ("(¬◡¬)✧", "more fab than you"),
        ("(^_<)〜☆", "wink, sparkle"),
        ("─=≡Σ((( つ＞＜)つ", "running"),
        ("ヽ（　＾＾）人（＾＾　）ノ", "high five"),
        ("(¬_¬'')ԅ(￣ε￣ԅ)", "secret"),
        ("(ﾉ≧∀≦)ﾉ ‥…━━━★", "wizard beam attack"),
        ("¯\\_(ツ)_/¯", "shrug"),
        ("♡₊˚", "hearts and sparkles"),
        ("(◕‿◕)♡", "love"),
        ("ᨐᵉᵒʷ", "meow"),
        ("♡( ◡‿◡ )", "love"),
        ("(´,,•ω•,,)♡", "love, blushing"),
        (">〴⋋_⋌〵 ", "rage, cat, angry"),
        (" (ꈍᴗꈍ)ε｀*)", "kiss, blushing"),
        ("(ɔˆ ³(ˆ⌣ˆc)", "kiss, smiling"),
        ("(˵ᵕ̴᷄ ˶̫ ˶ᵕ̴᷅˵)", "flushed, blushing"),
        ("‧⁺✧(˶´⚰︎`˵)⁺‧", "flushed, blushing, sparkles"),
        ("(●｀・ω・)=Ｏ)｀-д゜)", "punch"),
        ("^_^", "smile"),
        ("T_T", "crying"),
    ]
    .iter()
    .map(|row| Moji {
        value: row.0.into(),
        description: row.1.into(),
    })
    .collect::<Vec<Moji>>()
}
