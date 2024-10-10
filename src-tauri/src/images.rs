use std::{fs, io::Write};

use crate::fs_utils::get_unique_path;

// Write an image into the app data directory
pub fn save_image<'a>(
    data: &'a [u8],
    extension: &str,
    image_cache_dir: &str,
) -> Result<String, String> {
    // Create a path that does not exist already
    let path = get_unique_path(image_cache_dir, extension)?;
    println!("Path: {}", path);

    // Write file, handle errors
    let file = fs::OpenOptions::new().create(true).write(true).open(&path);

    let Ok(mut file) = file else { return Err("Couldn't open file".into()); };

    if let Ok(_) = file.write_all(data) {
        return Ok(path);
    }
    Err("Unknown error".into())
}

// Wrapper
pub fn save_cover<'a>(data: &'a [u8], extension: &str, image_cache_dir: &str) -> Option<String> {
    match save_image(data, extension, image_cache_dir) {
        Ok(value) => Some(value),
        Err(error) => {
            println!("Error in save_cover: {}", error);
            None
        }
    }
}
