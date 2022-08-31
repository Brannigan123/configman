use crate::config::SAMPLE_CONFIG_CONTENT;
use crate::fs::get_working_dir;

use git2::Repository;
use std::fs;

/// It checks if a configuration file exists in the current working directory, and if it doesn't, it
/// creates one
pub fn init_working_dir() {
    let config_path = get_working_dir().join("config.cmf");
    if !&config_path.exists() {
        println!("No configuration file found in current working directory.");
        match fs::write(&config_path, SAMPLE_CONFIG_CONTENT) {
            Ok(_) => println!("Generated a new config file at: {:?}", &config_path),
            Err(e) => panic!("Failed to generated new config file: {:?}", e),
        }
    }
    println!("Using config file: {:?}", &config_path);
}

pub fn ensure_is_git_working_dir() -> Repository {
    Repository::init(get_working_dir()).expect("Failed to initialize git repo")
}

pub fn clone_from_remote(url: &str) -> Repository {
    Repository::clone(url, get_working_dir()).expect("Failed to clone repo")
}
