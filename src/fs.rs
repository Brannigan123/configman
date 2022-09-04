use crate::git::{get_file_status, rm_file};
use capturing_glob::{glob_with, Entry, MatchOptions, PatternError};
use indicatif::{ProgressBar, ProgressIterator};
use std::{env, fs, path::PathBuf};

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

/// It removes all files and directories from the working directory except for the `config.cmf` file,
/// the `.git` directory and any files ignored by git
pub fn clean_working_dir() {
    let wdir = get_working_dir();
    let skip = vec![
        wdir.join("config.cmf"),
        wdir.join(".git"),
        wdir.join("LICENSE"),
    ];
    let rdir = wdir.read_dir().expect("Failed read from working directory");
    for entry in
        rdir.progress_with(ProgressBar::new_spinner().with_message("Cleaning working directory"))
    {
        if let Ok(dir_entry) = entry {
            let entry_path = dir_entry.path();
            let ignored = skip.contains(&entry_path)
                || get_file_status(&entry_path.display().to_string()).is_ignored();
            if !ignored {
                remove_from_fs(&entry_path);
                rm_file(&entry_path.display().to_string());
            }
        }
    }
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

/// If the original path is a directory, create a symbolic link, otherwise create a hard link
///
/// Arguments:
///
/// * `original`: The path to the original file or directory.
/// * `link`: The path to the link to be created.
pub fn link_path(original: &PathBuf, link: &PathBuf) {
    if original.is_dir() {
        std::os::unix::fs::symlink(original, link)
    } else {
        fs::hard_link(original, link)
    }
    .expect(format!("Failed to link to {:?} as {:?}", &original, &link).as_str())
}
