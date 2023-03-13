mod config;
mod moji;
mod rofi;

fn main() {
    let cfg = config::load_config();
    let moji = moji::get_moji_list();
    let index = match rofi::get_index(&cfg, &moji::moji_to_string(&moji)) {
        Err(reason) => panic!("rofi failed because: {}", reason),
        Ok(index) => index,
    };
    print!("{}", moji[index].value);
}
