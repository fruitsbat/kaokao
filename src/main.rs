mod config;
mod moji;
mod rofi;

fn main() {
    let cfg = config::load_config();
    let mut mojilist = match moji::get_moji_list(&cfg) {
        Err(reason) => panic!("could not get emoji list because {}", reason),
        Ok(m) => m,
    };
    let mut index = match rofi::get_index(&cfg, &moji::moji_to_string(&mojilist)) {
        Err(reason) => panic!("rofi failed because: {}", reason),
        Ok(index) => index,
    };

    // the moji might have a variation!
    if !mojilist[index].variations.is_empty() {
        mojilist = mojilist[index].variations.clone();
        // open rofi again to pick variation
        index = match rofi::get_index(&cfg, &moji::moji_to_string(&mojilist)) {
            Err(reason) => panic!("rofi failed because: {}", reason),
            Ok(index) => index,
        };
    }

    match moji::recent::save(mojilist[index].clone()) {
        Ok(_) => (),
        Err(reason) => panic!("could not save most recently used because: {}", reason),
    }
    print!("{}", mojilist[index].value);
}
