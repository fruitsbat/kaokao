use std::{
    error::Error,
    io::{self, prelude, Read, Write},
    process::{Command, Stdio},
};

pub fn get_index() -> Result<u64, Box<dyn Error>> {
    // spawn rofi command
    let rofi = Command::new("rofi")
        .arg("-format")
        .arg("i")
        .arg("-dmenu")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // write list to rofi
    rofi.stdin
        .unwrap()
        .write_all("meow\nkitty\nmeow".as_bytes())?;

    // read to index
    let mut index = String::new();
    rofi.stdout
        .unwrap()
        .read_to_string(&mut index)
        .expect("failed to read from rofi");

    let index: u64 = index.trim().parse()?;

    Ok(index)
}
