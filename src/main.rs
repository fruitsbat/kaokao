use clap::Parser;
use rand::{distributions::WeightedIndex, prelude::*};
use std::{fs, path::PathBuf};
use strum::IntoEnumIterator;

mod builtins;
mod print;

// test the example with `cargo run --example most_simple`
fn main() {
    let args = Args::parse();
    let files = read_path(&args);

    let builtin_count = if args.no_builtins {
        0
    } else {
        if args.builtins.is_empty() {
            builtins::Builtins::iter().count()
        } else {
            args.builtins.len()
        }
    };

    if files.len() + if args.no_builtins { 0 } else { builtin_count } <= 0 {
        println!("nothing to see here...\ncould not find any input files");
        return;
    }

    // weighting
    let builtin_or_file = [DataType::Builtin, DataType::File];
    let weights = [builtin_count, files.len()];
    let dist = WeightedIndex::new(&weights).expect("failed to parse weighted index list!");

    let mut rng = rand::thread_rng();
    let builtin_or_file = &builtin_or_file[dist.sample(&mut rng)];
    let data = match builtin_or_file {
        DataType::Builtin => {
            if args.builtins.is_empty() {
                // we pick from all of the builtins!
                builtins::Builtins::iter()
                    .choose(&mut rng)
                    .unwrap_or(builtins::Builtins::KittyFace)
                    .get()
                    .into()
            } else {
                // we pick from a specified list of builtins
                args.builtins
                    .choose(&mut rng)
                    .unwrap_or(&builtins::Builtins::KittyFace)
                    .get()
                    .into()
            }
        }
        DataType::File => {
            fs::read_to_string(files.choose(&mut rng).expect("failed to pick a file"))
                .expect("failed to read file")
        }
    };
    print::print(data, &args);
}

enum DataType {
    Builtin,
    File,
}

fn read_path(args: &Args) -> Vec<PathBuf> {
    let mut bufs = vec![];
    for path in &args.paths {
        let entries = PathBuf::from(path);
        let pathtype = entries
            .metadata()
            .expect(&format!("{} seems to be broken, sorry!", &path));
        if pathtype.is_file() {
            bufs.push(entries);
            continue;
        }

        let mut entries = fs::read_dir(entries)
            .expect("Failed to read folder!")
            .filter(|e| e.is_ok())
            .map(|e| e.unwrap())
            // filter unreadable paths
            // folders are not useful to us
            .filter(|e| e.metadata().is_ok())
            .filter(|e| e.metadata().unwrap().file_type().is_file())
            .map(|e| e.path())
            .collect::<Vec<PathBuf>>();
        bufs.append(&mut entries)
    }
    bufs
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// how many times it will be printed horizontally
    #[arg(short, long, default_value_t = 1)]
    rows: u8,

    /// how many times it will be printed vertically
    #[arg(short, long, default_value_t = 6)]
    columns: u8,

    /// which mode to use
    #[arg(long, value_enum, num_args = 1.., value_delimiter = ' ')]
    color_modes: Vec<print::ColorMode>,

    /**
    color to start with, uses random if not set
    0 = black
    1 = red
    2 = green
    3 = yellow
    4 = blue
    5 = magenta
    6 = cyan
    7 = white
    */
    #[arg(long)]
    color_offset: Option<u8>,

    /// add a folder or file to the set of ascii art
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    paths: Vec<String>,

    /**
    don't use built in art
    overrides the builtins option
    */
    #[arg(long, default_value_t = false)]
    no_builtins: bool,

    /// pick specific builtins you want to add to the set
    #[arg(long, value_parser, num_args = 1.., value_delimiter = ' ')]
    builtins: Vec<builtins::Builtins>,
}
