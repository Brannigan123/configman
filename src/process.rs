use crate::config::SAMPLE_CONFIG_CONTENT;
use crate::fs::get_working_dir;
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
