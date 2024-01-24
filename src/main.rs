use walkdir::WalkDir;

fn main() {
    let walker = WalkDir::new(r"C:\Users\pc\Downloads\test").into_iter();
    for entry in walker
        .filter_entry(|e| {
            //println!("{:?}", e.file_name().to_str());
            e.file_name()
                .to_str()
                .map(|s| {
                    println!("{}", s.contains("y"));
                    s.contains("y")
                })
                .unwrap_or(false)
        })
        .filter_map(|v| v.ok())
    {
        println!("{}", entry.path().display());
    }
}
