use capturing_glob::{glob_with, Entry, MatchOptions, PatternError};
use std::{
    env,
    fs::{self, File},
    path::PathBuf,
};

/// Setting the options for the globbing.
const OPTIONS: MatchOptions = MatchOptions {
    case_sensitive: true,
    require_literal_separator: false,
    require_literal_leading_dot: false,
};

/// It takes a glob pattern, and returns a list of files that match that pattern
///
/// Arguments:
///
/// * `pattern`: The pattern to match.
///
/// Returns:
///
/// A vector of entries.
pub fn get_matching_files(pattern: &str) -> Result<Vec<Entry>, PatternError> {
    glob_with(pattern, &OPTIONS).map(|ps| ps.map(|p| p.unwrap()).collect::<Vec<Entry>>())
}

/// `get_working_dir()` returns the current working directory
///
/// Returns:
///
/// A Result<PathBuf>
pub fn get_working_dir() -> PathBuf {
    env::current_dir().expect("Failed to get current working directory.")
}

/// `create_file` creates a file at the given path, creating any parent directories if necessary
///
/// Arguments:
///
/// * `path`: The path to the file to create.
///
/// Returns:
///
/// A Result<File, std::io::Error>
pub fn create_file(path: &PathBuf) -> Result<File, std::io::Error> {
    path.parent()
        .map(|parent| fs::create_dir_all(parent))
        .map(|r| match r {
            Ok(_) => File::create(path),
            Err(e) => Err(e),
        })
        .unwrap_or_else(|| File::create(path))
}

/// It removes a file or directory from the filesystem
/// 
/// Arguments:
/// 
/// * `path`: The path to the file or directory to remove.
pub fn remove_from_fs(path: &PathBuf) {
    if path.is_dir() {
        fs::remove_dir_all(&path).ok();
    } else if path.exists() {
        fs::remove_file(&path).ok();
    }
}