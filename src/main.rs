mod config;
mod rofi;

fn main() {
    let cfg = config::load_config();
    let index = match rofi::get_index(&cfg) {
        Err(reason) => panic!("rofi failed because: {}", reason),
        Ok(index) => index,
    };
    println!("index: {}", index);
}
