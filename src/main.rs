use std::fs;

use rayon::prelude::*;
use walkdir::WalkDir;

fn main() {
    println!("Hello, world!");
    let path = r#"C:\test"#;
    scan_directory(path)

}

fn scan_directory(path: &str) {
    // Create a new WalkDir iterator for the given path
    WalkDir::new(path)
    // Get an iterator of directory entries and filter out any errors
    .into_iter()
    .filter_map(Result::ok)
    // Convert the iterator to a parallel iterator for better performance
    .par_bridge()
    // For each file print path
    .for_each(|entry| {
        let path = entry.path();
        if path.is_file() {
            if let Ok(metadata) = fs::metadata(path) {
                // add size
                let size = metadata.len();
                println!("{} ({} bytes)", path.display(),size);

            }
            
            
        }
    }) 
    
}