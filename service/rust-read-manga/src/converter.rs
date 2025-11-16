use crate::cli::CliArgs;
use crate::error::AppError;
use std::fs;
use std::path::PathBuf;

// Define the extensions we support
const ALLOWED_EXTENSIONS: [&str; 3] = ["png", "jpg", "jpeg"];

pub fn run_conversion(args: CliArgs) -> Result<(), AppError> {
    // 1. Find and sort images in 'args.input_dir'.
    println!("Scanning input directory: {:?}", args.input_dir);

    // Read all entries in the directory
    let mut image_paths: Vec<PathBuf> = fs::read_dir(&args.input_dir)?
        .filter_map(Result::ok) // Filter out any read errors for individual entries
        .map(|entry| entry.path()) // Get the PathBuf for each entry
        .filter(|path| {
            // Check if it's a file and has an allowed extension
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    return ALLOWED_EXTENSIONS.contains(&ext.to_lowercase().as_str());
                }
            }
            false
        })
        .collect();

    if image_paths.is_empty() {
        return Err(AppError::NoImagesFound(args.input_dir));
    }

    // 2. Sort the images alphanumerically
    // This is a natural sort, which is what we want for "page_1, page_2, page_10"
    image_paths.sort();

    println!("Found and sorted {} images:", image_paths.len());
    // (For testing) Print the first 10 images found
    for path in image_paths.iter().take(10) {
        println!(" - {:?}", path.file_name().unwrap_or_default());
    }
    if image_paths.len() > 10 {
        println!(" ...and {} more.", image_paths.len() - 10);
    }
    
    // ---
    // 3. TODO: Initialize FFmpeg encoder (Next Step)
    // ---

    // ---
    // 4. TODO: Loop through 'image_paths' and encode each frame
    // ---

    Ok(())
}