use std::fs;
use std::time::{Instant};

use rayon::prelude::*;
use walkdir::WalkDir;

fn main() {
    let start = Instant::now();
    println!("Hello, world!");
    let path = r#"C:\test"#;
    scan_directory(path);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

}
/// Recursively scans a directory and prints the size of each file in a readable format.
///
/// # Arguments
///
/// * `path` - A string slice that represents the path of the directory to scan.
///
/// # Examples
///
/// ```
/// scan_directory("C:\\test");
/// ```
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

/// Convert the given size in bytes to a human-readable format with appropriate units (KB, MB, GB, TB).
///
/// # Arguments
///
/// * `size`  - The size in bytes to be converted to a human-readable format.
/// 
/// # Returns
/// 
/// * A String containing the human-readable size with appropriate units.
///
/// # Examples
///```
/// let size = 1500000000;
/// let readable_size = size_readable_format(size);
/// println!("{}", readable_size); // prints "1.40 GB"
/// ```
fn size_readable_format(size: u64) -> String {
    // Sets size conversions
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


// Tests
#[test]
fn test_size_readable_format() {
    assert_eq!(size_readable_format(1023), "1023 B");
    assert_eq!(size_readable_format(1024), "1.00 KB");
    assert_eq!(size_readable_format(1024 * 1024), "1.00 MB");
    assert_eq!(size_readable_format(1024 * 1024 * 1024), "1.00 GB");
    assert_eq!(size_readable_format(1024 * 1024 * 1024 * 1024), "1.00 TB");
}