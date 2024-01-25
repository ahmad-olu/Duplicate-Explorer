use std::fs;
use winwalk::*;

fn main() {
    for file in walkdir(r"C:\Users\pc\Downloads\test", 1)
        .into_iter()
        .flatten()
    {
        if file.name.starts_with("y2mate.com - a - ") {
            //println!("{:#?}", file);
            //println!("{}", file.path);
            let new_name_path = file.path.replace("y2mate.com - a - ", "a");
            fs::rename(file.path, new_name_path).expect("error renaming file");
        }
    }
}
