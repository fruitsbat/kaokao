mod config;
mod moji;
mod rofi;

fn main() {
    let cfg = config::load_config();
    let mojilist = match moji::get_moji_list(&cfg) {
        Err(reason) => panic!("could not get emoji list because {}", reason),
        Ok(m) => m,
    };
    let index = match rofi::get_index(&cfg, &moji::moji_to_string(&mojilist)) {
        Err(reason) => panic!("rofi failed because: {}", reason),
        Ok(index) => index,
    };
    match moji::save_recent(mojilist[index].clone()) {
        Ok(_) => (),
        Err(reason) => panic!("could not save most recently used because: {}", reason),
    }
    print!("{}", mojilist[index].value);
}
