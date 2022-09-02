use capturing_glob::Entry;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::config::Mapping;
use crate::config::SAMPLE_CONFIG_CONTENT;
use crate::fs::{get_matching_files, get_working_dir};
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
    add_file(&config_path.display().to_string());
    println!("Using config file: {:?}", &config_path);
}

/// It takes a vector of mappings, and for each mapping, it ensures that the destination exists as a hardlink to
/// the source
///
/// Arguments:
///
/// * `mappings`: A vector of Mapping structs.
pub fn link_mappings(mappings: &Vec<Mapping>) -> Vec<PathBuf> {
    let mut links = Vec::new();
    for mapping in mappings {
        let src = &mapping.source;
        let dest = &mapping.destination;
        let original = PathBuf::from(&src);
        let link = if Path::new(&dest).is_absolute() {
            PathBuf::from(&dest)
        } else {
            get_working_dir().join(&dest)
        };
        ensure_link_upto_date(&original, &link);
        links.push(link.to_owned());
    }
    links
}

/// It creates the parent directory of the link if it doesn't exist, and then creates a hard link from
/// the original file to the link
///
/// Arguments:
///
/// * `original`: The path to the original file.
/// * `link`: The path to the link to be created.
pub fn ensure_link_upto_date(original: &PathBuf, link: &PathBuf) {
    link.parent().map(|p| fs::create_dir_all(p));
    if link.is_dir() {
        fs::remove_dir_all(&link).ok();
    } else if link.exists() {
        fs::remove_file(&link).ok();
    }
    fs::hard_link(original, link).expect(format!("Failed to link to {:?}", &original).as_str());
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
    for mapping in &config.mappings {
        for matched in &get_matching_files(&mapping.source).expect("Failed match files") {
            let source = matched.path().display().to_string();
            let destination = substitute_group_values(mapping, matched);
            found_mappings.push(Mapping {
                source,
                destination,
            });
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
