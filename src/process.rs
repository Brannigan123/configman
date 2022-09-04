use capturing_glob::Entry;
use indicatif::{ProgressIterator, ProgressStyle};
use same_file::is_same_file;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::config::Mapping;
use crate::config::SAMPLE_CONFIG_CONTENT;
use crate::fs::{get_matching_files, get_working_dir, link_path, remove_from_fs};
use crate::git::{add_file, init_git, is_git_repo_root_dir};

/// It checks if the current working directory has a configuration file, if not, it generates one, then
/// it checks if the current working directory is a git repository, if not, it initializes one, then it
/// adds the configuration file to the git repository
pub fn init_working_dir() {
    let config_path = get_working_dir().join("config.cmf");
    if !&config_path.exists() {
        println!("No configuration file found in current working directory.");
        match fs::write(&config_path, SAMPLE_CONFIG_CONTENT) {
            Ok(_) => println!("Generated a new config file at: {:?}", &config_path),
            Err(e) => panic!("Failed to generated new config file: {:?}", e),
        }
    }
    if !is_git_repo_root_dir() {
        init_git();
    }
    add_file(&vec![config_path.display().to_string()]);
}

/// It takes a vector of paths, converts them to strings, and then sends them to the `add_file` function
/// in batches of 16
///
/// Arguments:
///
/// * `paths`: A vector of PathBufs that we want to index.
pub fn track_links(paths: &Vec<PathBuf>) {
    let path_strs = paths
        .iter()
        .map(|p| p.display().to_string())
        .collect::<Vec<String>>();
    for batch in path_strs
        .chunks(16)
        .progress_with_style(
            ProgressStyle::with_template("[{percent}%]{prefix} {wide_bar} eta: {eta_precise}")
                .unwrap(),
        )
        .with_prefix("Indexing files")
    {
        add_file(&batch.to_vec());
    }
}

/// It takes a vector of mappings, and for each mapping, it ensures that the destination exists as a hardlink to
/// the source
///
/// Arguments:
///
/// * `mappings`: A vector of Mapping structs.
///
/// Returns:
///
/// A vector of PathBufs
pub fn link_mappings(mappings: &Vec<Mapping>) -> Vec<PathBuf> {
    let mut linked = Vec::new();
    for mapping in mappings
        .iter()
        .progress_with_style(
            ProgressStyle::with_template("[{percent}%]{prefix} {wide_bar} eta: {eta_precise}")
                .unwrap(),
        )
        .with_prefix("Linking files")
    {
        let src = &mapping.source;
        let dest = &mapping.destination;
        let original = PathBuf::from(&src);
        let link = if Path::new(&dest).is_absolute() {
            PathBuf::from(&dest)
        } else {
            get_working_dir().join(&dest)
        };
        if ensure_link_upto_date(&original, &link) {
            linked.push(link);
        }
    }
    return linked;
}

/// If the original file exists, then if the link exists, replace it with a new link, else create a new
/// link
///
/// Arguments:
///
/// * `original`: The original file that you want to link to.
/// * `link`: The path to the link to be created
///
/// Returns:
///
/// A boolean value. whether it created a new link
pub fn ensure_link_upto_date(original: &PathBuf, link: &PathBuf) -> bool {
    if original.exists()
    /* Helps in ignoring broken links */
    {
        if link.exists() {
            replace_existing_with_link(&original, &link)
        } else {
            create_new_link(&original, &link);
        }
        return true;
    }
    return false;
}

/// If the link already exists, and it's not the same file as the original, then remove it and replace
/// it with a link to the original unless it's inside a symbolic ancestor pointing to folder outside the
/// working directory
///
/// Arguments:
///
/// * `original`: The path to the file that you want to link to.
/// * `link`: The path to the link to be created
fn replace_existing_with_link(original: &PathBuf, link: &PathBuf) {
    let same =
        is_same_file(original.display().to_string(), link.display().to_string()).unwrap_or(false);
    if !same {
        if link.canonicalize().unwrap().starts_with(get_working_dir()) {
            remove_from_fs(&link);
            link_path(&original, &link);
        } else {
            panic!(
                "Failed to link {:?} as {:?}, since a different file already exists there",
                &original, &link
            );
        }
    }
}

/// It creates a new link at the given path
///
/// Arguments:
///
/// * `original`: The path to the original file.
/// * `link`: The path to the new link
fn create_new_link(original: &PathBuf, link: &PathBuf) {
    link.parent().map(|p| fs::create_dir_all(p));
    link_path(&original, &link);
}

/// It takes a `Config` and returns a `Vec<Mapping>` where each `Mapping` is a source and destination
/// file path that have been matched by the config mappings
///
/// Arguments:
///
/// * `config`: &Config
///
/// Returns:
///
/// A vector of Mapping structs.
pub fn get_found_mappings(config: &Config) -> Vec<Mapping> {
    let mut found_mappings = Vec::new();
    for mapping in config
        .mappings
        .iter()
        .progress_with_style(
            ProgressStyle::with_template("[{percent}%]{prefix} {wide_bar} eta: {eta_precise}")
                .unwrap(),
        )
        .with_prefix("Finding matching files")
    {
        for matched in &get_matching_files(&mapping.source).expect("Failed match files") {
            if !matched.path().is_dir() {
                let source = matched.path().display().to_string();
                let destination = substitute_group_values(mapping, matched);
                found_mappings.push(Mapping {
                    source,
                    destination,
                });
            }
        }
    }
    return found_mappings;
}

/// It takes a mapping and a matched entry, and returns a string with all the group values substituted
/// in the destination
///
/// Arguments:
///
/// * `mapping`: The mapping that we're using to transform the source string.
/// * `matched`: The entry that matched the mapping.
///
/// Returns:
///
/// A String
fn substitute_group_values(mapping: &Mapping, matched: &Entry) -> String {
    let mut destination = mapping.destination.clone();
    let mut group_index: usize = 1;
    loop {
        match substitute_group_value(&destination, &matched, group_index) {
            Some((dest, grp_index)) => {
                destination = dest;
                group_index = grp_index;
            }
            None => break,
        }
    }
    destination
}

/// It takes a destination string, a matched entry, and a position, and returns a tuple of the
/// destination string with the group at the given position substituted in, and the next position
///
/// Arguments:
///
/// * `destination`: The destination string that we're going to replace the group values in.
/// * `matched`: The matched entry
/// * `position`: The position of the group in the regex.
///
/// Returns:
///
/// A tuple of the new destination and the next position to substitute.
fn substitute_group_value(
    destination: &String,
    matched: &Entry,
    position: usize,
) -> Option<(String, usize)> {
    matched.group(position).map(|group| {
        let old = format!("({})", position);
        (
            destination.replace(old.as_str(), group.to_str().unwrap()),
            position + 1,
        )
    })
}
