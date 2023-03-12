use clap::ValueEnum;
use colored::*;
use pad::PadStr;
use rand::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

/// give number of rainbow color n this returns string in that color
fn rainbowify<S: Into<String>>(input: S, number: u64) -> ColoredString {
    match number.rem_euclid(8) {
        0 => input.into().black(),
        1 => input.into().red(),
        2 => input.into().green(),
        3 => input.into().yellow(),
        4 => input.into().blue(),
        5 => input.into().magenta(),
        6 => input.into().cyan(),
        7 => input.into().white(),
        // waht the fuck
        _ => input.into().red(),
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, EnumIter)]
pub enum ColorMode {
    Solid,
    Rainbow,
    Uniform,
    Trans,
}

pub fn print<S: Into<String>>(input: S, args: &crate::Args) {
    let mut rng = thread_rng();
    let rows = args.rows;
    let columns = args.columns;
    let input: String = input.into();
    let mut longest: usize = 0;
    let offset = args
        .color_offset
        .unwrap_or(rand::thread_rng().gen_range(0..5));

    for line in input.lines() {
        if line.chars().count() > longest {
            longest = line.chars().count();
        }
    }

    let colormode: ColorMode = if args.color_modes.is_empty() {
        ColorMode::iter()
            .choose(&mut rng)
            .expect("failed to pick color mode!")
    } else {
        args.color_modes
            .choose(&mut rng)
            .expect("failed to pick color mode!")
            .to_owned()
    };

    let mut linecount: u64 = 0;
    for row in 0..rows {
        for line in input.lines() {
            linecount += 1;
            for column in 0..columns {
                let color = (match colormode {
                    ColorMode::Solid => {
                        column.wrapping_add(row * columns).wrapping_add(offset) as u64
                    }
                    ColorMode::Uniform => offset as u64,
                    ColorMode::Rainbow => linecount.wrapping_add(offset as u64) as u64,
                    ColorMode::Trans => match column.rem_euclid(5) {
                        0 => 4,
                        1 => 5,
                        2 => 7,
                        3 => 5,
                        4 => 4,
                        _ => 0,
                    },
                }) as u64;
                if column == columns - 1 {
                    println!("{}", rainbowify(line, color));
                } else {
                    print!("{}", rainbowify(line.pad_to_width(longest), color))
                }
            }
        }
    }
}
