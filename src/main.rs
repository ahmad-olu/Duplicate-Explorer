use clap::{arg, command, Parser};
use std::fs;
use winwalk::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the type of feature
    #[arg(short, long)]
    feature: String,
    /// the path to read
    #[arg(short, long)]
    path: String,

    /// the value to be replaces
    #[arg(short, long)]
    contains: String,

    /// What to replace it with
    #[arg(short, long)]
    replace_with: String,
}

fn main() {
    //cargo run -- -f hello
    let args = Args::parse();

    if args.feature.to_lowercase() == "replace" {
        find_and_rename(
            args.path.as_str(),
            args.contains.as_str(),
            args.replace_with.as_str(),
        )
    }
}

fn find_and_rename(path: &str, contains: &str, replace_with: &str) {
    //path = r"C:\Users\pc\Downloads\test"
    //contains = "y2mate.com - a - "
    //replace_with = "a"
    for file in walkdir(path, 0).into_iter().flatten() {
        if file.name.contains(contains) {
            //println!("{:#?}", file);
            //println!("{}", file.path);
            let new_name_path = file.path.replace(contains, replace_with);
            fs::rename(file.path, new_name_path).expect("error renaming file");
        }
    }
}
