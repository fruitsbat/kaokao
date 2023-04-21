use crate::{config::Config, moji::Moji};
use std::{error::Error, fs};

/// load an emoji from a file list
pub fn load_moji_from_files(cfg: &Config) -> Result<Vec<Moji>, Box<dyn Error>> {
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

#[cfg(test)]
mod tests {
    
    #[test]
    fn test_parsing() {}
}
