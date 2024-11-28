use std::{
    fs::{self, File},
    io::Write,
};

use image::{imageops::FilterType, load_from_memory_with_format, ImageFormat};

use crate::fs_utils::get_unique_path;

// Saves image in different sizes and returns their paths in a tuple
pub fn save_cover<'a>(
    data: &'a [u8],
    extension: &str,
    image_cache_dir: &str,
) -> (Option<String>, Option<String>, Option<String>) {
    let format: ImageFormat = match &extension.to_lowercase()[..] {
        "jpg" => ImageFormat::Jpeg,
        "jpeg" => ImageFormat::Jpeg,
        "png" => ImageFormat::Png,
        "gif" => ImageFormat::Gif,
        "bmp" => ImageFormat::Bmp,
        _ => ImageFormat::Jpeg,
    };

    // Create paths that do not exist already
    let Ok(path) = get_unique_path(image_cache_dir, extension) else { return (None, None, None) };
    let Ok(path_small) = get_unique_path(image_cache_dir, extension) else { return (None, None, None) };
    let Ok(path_tiny) = get_unique_path(image_cache_dir, extension) else { return (None, None, None) };

    // Save downscaled versions
    let Ok(img) = load_from_memory_with_format(data, format) else { return (None, None, None) };
    let small = img.resize(256, 256, FilterType::Triangle);
    let tiny = img.resize(128, 128, FilterType::Triangle);

    let Ok(mut output) = File::create(path_small.clone()) else { return (None, None, None) };
    if let Err(err) = small.write_to(&mut output, ImageFormat::Png) {
        println!("Error in save_cover (small), {}", err);
        return (None, None, None);
    }

    let Ok(mut output) = File::create(path_tiny.clone()) else { return (None, None, None) };
    if let Err(err) = tiny.write_to(&mut output, ImageFormat::Png) {
        println!("Error in save_cover (original), {}", err);
        return (None, None, None);
    }

    // Save original version
    let file = fs::OpenOptions::new().create(true).write(true).open(&path);
    let Ok(mut file) = file else { return (None, None, None) };
    if let Err(err) = file.write_all(data) {
        println!("Error in save_cover (original), {}", err);
        return (None, None, None);
    }

    (Some(path), Some(path_small), Some(path_tiny))
}
