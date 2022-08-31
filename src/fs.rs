use capturing_glob::{glob_with, Entry, MatchOptions, PatternError};
use std::{env, path::PathBuf};

/// Setting the options for the globbing.
const OPTIONS: MatchOptions = MatchOptions {
    case_sensitive: false,
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
pub fn get_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}
