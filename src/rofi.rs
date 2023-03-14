use std::{
    error::Error,
    io::{Read, Write},
    process::{Command, Stdio},
};

pub fn get_index(config: &crate::config::Config, list: &String) -> Result<usize, Box<dyn Error>> {
    // spawn rofi command
    let rofi = Command::new(&config.rofi_binary)
        .arg("-format")
        .arg("i")
        .arg("-dmenu")
        .arg("-p")
        .arg(&config.prompt)
        .arg("-i")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // write list to rofi
    rofi.stdin.unwrap().write_all(list.as_bytes())?;

    // read to index
    let mut index = String::new();
    rofi.stdout
        .unwrap()
        .read_to_string(&mut index)
        .expect("failed to read from rofi");

    let index: usize = index.trim().parse()?;

    Ok(index)
}
