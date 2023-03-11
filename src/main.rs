use rayon::prelude::*;
use walkdir::WalkDir;

fn main() {
    println!("Hello, world!");
    let path = r#"C:\test"#;
    scan_directory(path)

}

fn scan_directory(path: &str) {
    WalkDir::new(path)
    .into_iter()
    .filter_map(Result::ok)
    .par_bridge()
    .for_each(|entry| {
        let path = entry.path();
        if path.is_file() {
            println!("{}", path.display());
        }
    }) 
    
}