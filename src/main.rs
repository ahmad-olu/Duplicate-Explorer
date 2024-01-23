use std::{fs::File, io::{BufRead, BufReader}};

use walkdir::{DirEntry, WalkDir};


// fn is_hidden(entry: &DirEntry) -> bool {
//     entry.file_name()
//          .to_str()
//          .map(|s| s.contains(r"y2mate.com - "))
//          .unwrap_or(false)
// }

fn main() {


let walker = WalkDir::new(r"C:\Users\pc\Downloads\test").into_iter();
    for entry in walker.filter_map(|e|e.ok()){
            if entry.file_type().is_file(){
                let file_path = entry.path();
                if let Ok(file)= File::open(&file_path){
                    let reader = BufReader::new(file);

                    for (line_number, line) in reader.lines().enumerate() {
                        if let Ok(line_content) = line{
                            if line_content.contains(r"y2mate.com"){
                                        println!("keyword found in {}: Line {} -{}",
                                            file_path.display(),
                                            line_number + 1,
                                            line_content
                        );
                            }
                        }
                    }   
                }else{
                    eprintln!("Error opening file: {}", file_path.display());
                }
            }
    }
}
