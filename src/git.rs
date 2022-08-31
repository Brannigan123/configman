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

/// `exec_git` takes a vector of strings and returns a `Result` of `Output` or `Error`
/// 
/// Arguments:
/// 
/// * `arg`: Vec<&str> - A vector of arguments to pass to the git command.
/// 
/// Returns:
/// 
/// A Result<Output, Error>
pub fn exec_git(arg: Vec<&str>) -> Result<Output, Error> {
    Command::new("git").args(arg).output()
}

/// If the output of `git rev-parse --git-dir` is `.git`, then we're in a git repo
/// 
/// Returns:
/// 
/// A boolean value.
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

/// It adds a file to the git index
/// 
/// Arguments:
/// 
/// * `path`: The path to the file to be indexed.
pub fn index_file(path: &str) {
    exec_git(vec!["add", &path, "-s"]).expect(format!("{}", &path).as_str());
}
