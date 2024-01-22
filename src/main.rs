use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

#[derive(Debug, Eq)]
struct FileInfo {
    size: u64,
    path: PathBuf,
}

impl PartialEq for FileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}

impl Hash for FileInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.size.hash(state);
    }
}

fn get_file_info(file_path: &Path) -> Option<FileInfo> {
    if let Ok(metadata) = fs::metadata(file_path) {
        if metadata.is_file() {
            return Some(FileInfo {
                size: metadata.len(),
                path: file_path.to_path_buf(),
            });
        }
    }
    None
}

fn find_duplicate_files(directory: &Path) {
    let mut file_info_map: HashMap<FileInfo, Vec<PathBuf>> = HashMap::new();

    for entry in fs::read_dir(directory).unwrap() {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            if let Some(file_info) = get_file_info(&file_path) {
                file_info_map
                    .entry(file_info)
                    .or_insert_with(Vec::new)
                    .push(file_path);
            }
        }
    }

    for (_, files) in file_info_map
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
    {
        println!("Duplicate files found:");
        for file in files {
            println!("{}", file.display());
        }
        println!("---");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        std::process::exit(1);
    }

    let directory = Path::new(&args[1]);
    if !directory.is_dir() {
        eprintln!("Error: {} is not a directory", directory.display());
        std::process::exit(1);
    }

    find_duplicate_files(directory);
}
