use std::io::Error;
use std::process::{Command, Output};

/// `GitFileStatus` is a struct that contains two fields, `index_status` and `working_tree_status`, both
/// of which are characters.
///
/// Properties:
///
/// * `index_status`: The status of the file in the index.
/// * `working_tree_status`: The status of the file in the working tree.
#[derive(Debug)]
pub struct GitFileStatus {
    pub index_status: char,
    pub working_tree_status: char,
}

impl GitFileStatus {
    /// `is_modified` returns true if the working tree status is modified is `M`
    ///
    /// Returns:
    ///
    /// A boolean value.
    pub const fn is_modified(&self) -> bool {
        self.working_tree_status == 'M'
    }

    /// `is_staged` returns true if the index status is `M`
    ///
    /// Returns:
    ///
    /// A boolean value.
    pub const fn is_staged(&self) -> bool {
        self.index_status == 'M'
    }

    /// `is_upto_date` returns true if the index and working tree are both up to date
    ///
    /// Returns:
    ///
    /// A boolean value.
    pub const fn is_upto_date(&self) -> bool {
        self.index_status == ' ' && self.working_tree_status == ' '
    }

    /// `is_untracked` returns true if the index status and working tree status are both `?`
    ///
    /// Returns:
    ///
    /// A boolean value.
    pub const fn is_untracked(&self) -> bool {
        self.index_status == '?' && self.working_tree_status == '?'
    }

    /// If the index status and the working tree status are both `!`, then the file is ignored
    ///
    /// Returns:
    ///
    /// A boolean value.
    pub const fn is_ignored(&self) -> bool {
        self.index_status == '!' && self.working_tree_status == '!'
    }
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
pub fn add_file(path: &str) {
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
            .map(|s| if s.is_empty() { "  " } else { s })
            .unwrap()
            .chars()
            .collect::<Vec<char>>();
        GitFileStatus {
            index_status: status[0],
            working_tree_status: status[1],
        }
    })
}

/// It runs `git status -s` and returns true if any of the lines start with `M`
///
/// Returns:
///
/// A boolean value.
pub fn is_any_file_staged() -> bool {
    exec_git(vec!["status", "-s"])
        .map(|output| {
            std::str::from_utf8(&output.stdout)
                .unwrap_or("")
                .lines()
                .any(|l| l.starts_with("M"))
        })
        .expect("Failed to determine if files have been staged")
}

/// It commits all staged files with the given message
/// 
/// Arguments:
/// 
/// * `message`: &str
pub fn commit_staged_files(message: &str) {
    if is_any_file_staged() {
        exec_git(vec!["commit", "-m", &message])
            .expect(format!("Failed to commit: {}", &message).as_str());
    } else {
        println!("There are no staged files. Commit has been aborted.");
    }
}
