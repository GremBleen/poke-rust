use directories::ProjectDirs;
use std::path::PathBuf;

pub fn cache_dir() -> PathBuf {
    ProjectDirs::from("", "", "poke-rust")
        .expect("no cache directory available on this platform")
        .cache_dir()
        .to_path_buf()
}