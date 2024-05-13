use std::path::PathBuf;

use audiotags::MimeType;
use rand::{distributions::Alphanumeric, Rng};

fn random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn get_unique_path(base: &str, extension: &str) -> Result<String, String> {
    let filename_len = 20;
    let mut max_iterations = 10000000;
    let path = PathBuf::from(base);

    loop {
        max_iterations -= 1;
        if max_iterations == 0 {
            return Err("Could not get an unique path, maximum amount of tries reached".into());
        }

        let candidate = path.join(format!("{}.{}", random_string(filename_len), extension));

        let Ok(exists) = candidate.try_exists() else { return Err("Failed to check if path exists in get_unique_path".into()); };

        if exists {
            continue;
        }

        return match candidate.to_str() {
            Some(path) => Ok(path.into()),
            None => Err("UTF-8 error when stringifying path".into()),
        };
    }
}

pub fn mime_type_to_extension<'a>(mime_type: MimeType) -> &'a str {
    match mime_type {
        MimeType::Png => "png",
        MimeType::Jpeg => "jpg",
        MimeType::Tiff => "tiff",
        MimeType::Bmp => "bpm",
        MimeType::Gif => "gif",
    }
}
