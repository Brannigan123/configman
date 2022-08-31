use crate::config::SAMPLE_CONFIG_CONTENT;
use crate::fs::get_working_dir;

use git2::Repository;
use std::fs;

/// It checks if a configuration file exists in the current working directory, and if it doesn't, it
/// creates one
pub fn init_working_dir() {
    let wd = get_working_dir();
    let config_path = wd.join("config.cmf");
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
    let wd = get_working_dir();
    Repository::init(wd).expect("Failed to initialize git repo")
}

pub fn clone_from_remote(url: &str) -> Repository {
    let wd = get_working_dir();
    Repository::clone(url, wd).expect("Failed to clone repo")
}
