use std::{
    io::{self, prelude, Read, Write},
    process::{Command, Stdio},
};

fn main() {
    // spawn rofi command
    let rofi = match Command::new("rofi")
        .arg("-format")
        .arg("i")
        .arg("-dmenu")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn rofi: {}", why),
        Ok(process) => process,
    };

    // write list to rofi
    match rofi
        .stdin
        .unwrap()
        .write_all("meow\nkitty\nmeow".as_bytes())
    {
        Err(why) => panic!("couldn't write to rofi stdin: {}", why),
        Ok(_) => (),
    }

    let mut index = String::new();
    rofi.stdout
        .unwrap()
        .read_to_string(&mut index)
        .expect("failed to read from rofi");

    let index = index.trim();
    println!("{}", index);
    let index: u64 = index
        .parse()
        .expect("couldn't convert rofi output to index");

    println!("picked index {}", index);
}
