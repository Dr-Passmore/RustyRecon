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
                println!("{} ({})", path.display(), size_readable_format(size));
            }
        }
    }) 
    
}

fn size_readable_format(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if size < KB {
        format!("{} B", size)
    } else if size < MB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else if size < GB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size < TB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else {
        format!("{:.2} TB", size as f64 / TB as f64)
    }
}