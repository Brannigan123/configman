use capturing_glob::Entry;

use crate::config::Config;
use crate::config::Mapping;
use crate::config::SAMPLE_CONFIG_CONTENT;
use crate::fs::{get_matching_files, get_working_dir};
use crate::git::{add_file, init_git, is_git_repo_root_dir};
use std::fs;

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
