use capturing_glob::{glob_with, Entry, MatchOptions, PatternError};

const OPTIONS: MatchOptions = MatchOptions {
    case_sensitive: false,
    require_literal_separator: false,
    require_literal_leading_dot: false,
};

pub fn get_matching_files(pattern: &str) -> Result<Vec<Entry>, PatternError> {
    glob_with(pattern, &OPTIONS).map(|ps| ps.map(|p| p.unwrap()).collect::<Vec<Entry>>())
}
