use std::io::Error;
use std::process::{Command, Output};

#[derive(Debug)]
enum GitFileStatus {
    Unmodified,
    Modified,
    TypeChanged,
    Added,
    Deleted,
    Renamed,
    Copied,
    Updated,
}

pub fn exec_git(arg: Vec<&str>) -> Result<Output, Error> {
    Command::new("git").args(arg).output()
}

pub fn is_git_repo_root_dir() -> bool {
    exec_git(vec!["rev-parse", "--git-dir"])
        .map(
            |output| match std::str::from_utf8(&output.stdout).unwrap().trim() {
                ".git" => true,
                _ => false,
            },
        )
        .expect("Failed to determine if current directory is a git repo")
}
