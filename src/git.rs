use std::io::Error;
use std::process::{Command, Output};

/// `GitFileStatus` is a struct with two fields, `IndexStatus` and `WorkingTreeStatus`, both of which
/// are `char`s.
///
/// Properties:
///
/// * `IndexStatus`: The status of the file in the index.
/// * `WorkingTreeStatus`: The status of the file in the working tree.
#[derive(Debug)]
pub struct GitFileStatus {
    pub IndexStatus: char,
    pub WorkingTreeStatus: char,
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
    exec_git(vec!["add", &path, "-s"]).expect(format!("Failed to index {}", &path).as_str());
}

/// It runs `git status -s <path>` and parses the output
///
/// Arguments:
///
/// * `path`: The path to the file you want to check the status of.
///
/// Returns:
///
/// A Result<GitFileStatus, Error>
pub fn get_file_status(path: &str) -> Result<GitFileStatus, Error> {
    exec_git(vec!["status", "-s", &path]).map(|output| {
        let status = std::str::from_utf8(&output.stdout)
            .unwrap()
            .split_whitespace()
            .nth(0)
            .unwrap_or("!!")
            .chars()
            .collect::<Vec<char>>();
        GitFileStatus {
            IndexStatus: status[0],
            WorkingTreeStatus: status[1],
        }
    })
}

pub fn is_any_file_staged() -> bool {
    exec_git(vec!["diff", "--cached", "--quiet"])
        .map(
            |output| match std::str::from_utf8(&output.stdout).unwrap().trim() {
                "yes" => true,
                "no" => false,
                other => panic!("Failed to understand output from git: {}", other),
            },
        )
        .expect("Failed to determine if files have been staged")
}

pub fn commit_staged_files(message: &str) {
    exec_git(vec!["commit", "-m", &message])
        .expect(format!("Failed to commit: {}", &message).as_str());
}
