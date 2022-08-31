use std::path::PathBuf;

use glob::glob_with;
use glob::MatchOptions;

const OPTIONS: MatchOptions = MatchOptions {
    case_sensitive: false,
    require_literal_separator: false,
    require_literal_leading_dot: false,
};

pub fn get_matching_files(pattern: &str) -> Result<Vec<PathBuf>, glob::PatternError> {
    glob_with(pattern, OPTIONS).map(|ps| ps.map(|p| p.unwrap()).collect::<Vec<PathBuf>>())
}


