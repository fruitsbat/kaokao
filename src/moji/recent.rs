use crate::moji::Moji;
use std::{error::Error, fs};

pub fn save(moji: Moji) -> Result<(), Box<dyn Error>> {
    let mut recents = vec![moji.clone()];
    recents.append(
        &mut load()?
            .iter()
            .filter(|&m| m.clone() != moji)
            .cloned()
            .collect::<Vec<Moji>>(),
    );
    let mut new_recents = vec![];
    for (i, r) in recents.into_iter().enumerate() {
        // limit to 100 most recent
        if i >= 100 {
            break;
        }
        new_recents.push(r);
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

pub fn load() -> Result<Vec<Moji>, Box<dyn Error>> {
    let base_dir = directories::BaseDirs::new().ok_or("could not load base dir")?;
    let path = base_dir.data_dir().join("kaokao/recent.json");
    if !path.exists() {
        return Ok(vec![]);
    }
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{config::Config, moji};

    #[test]
    #[ignore]
    fn save_and_load() {
        save(Moji {
            description: "hehe".into(),
            value: ":-)".into(),
            variations: vec![],
        })
        .unwrap();

        let recent = load().unwrap();
        assert!(!recent.is_empty());
    }

    #[test]
    #[ignore]
    fn without() {
        save(Moji {
            description: "cat".into(),
            value: "🐈".into(),
            variations: vec![],
        })
        .unwrap();

        let with = moji::get_moji_list(&Config {
            show_duplicates: true,
            ..Default::default()
        });

        let without = moji::get_moji_list(&Config {
            show_duplicates: false,
            ..Default::default()
        });

        assert!(with.unwrap().len() > without.unwrap().len());
    }
}
