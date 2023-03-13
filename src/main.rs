use std::{
    io::{self, prelude, Read, Write},
    process::{Command, Stdio},
};

mod rofi;

fn main() {
    let index = match rofi::get_index() {
        Err(reason) => panic!("rofi failed because: {}", reason),
        Ok(index) => index,
    };
    println!("index: {}", index);
}
