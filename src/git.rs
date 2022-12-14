use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Output, Stdio};

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

/// It executes the `git` command, captures its standard output, and prints each line of the output to
/// the console
///
/// Arguments:
///
/// * `arg`: Vec<&str> - This is the vector of arguments that we want to pass to the git command.
///
/// Returns:
///
/// Result<(), Error>
pub fn exec_git_with_logs(arg: Vec<&str>) -> Result<(), Error> {
    let stdout = Command::new("git")
        .args(arg)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    BufReader::new(stdout)
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));
    Ok(())
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

/// It executes the `git init` command
pub fn init_git() {
    exec_git_with_logs(vec!["init"]).expect("Failed to initialize git repo here");
}

/// It adds a file to the index if it's not already added
///
/// Arguments:
///
/// * `path`: &str
pub fn add_file(paths: &Vec<String>) {
    let mut arg = vec!["add"];
    arg.extend(paths.iter().map(|s| s.as_str()));
    exec_git(arg).expect(format!("Failed to index {:?}", &paths).as_str());
}

/// It takes a path as a string, and stages only modified and deleted files.
pub fn stage_files() {
    exec_git_with_logs(vec!["add", "-u"]).expect("Failed to stage changes");
}

/// It removes a file from the index
///
/// Arguments:
///
/// * `path`: The path to the file to be removed.
pub fn rm_file(path: &str) {
    exec_git(vec!["rm", &path]).expect(format!("Failed to unindex {}", &path).as_str());
}

/// It runs `git status -s <path>` and parses the output
///
/// Arguments:
///
/// * `path`: The path to the file you want to check the status of.
///
/// Returns:
///
/// GitFileStatus
pub fn get_file_status(path: &str) -> GitFileStatus {
    exec_git(vec!["status", "-s", &path])
        .map(|output| {
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
        .expect(format!("Failed to get git status: {}", path).as_str())
}

/// It runs `git status` and checks if the output contains the string `Changes to be committed:`
///
/// Returns:
///
/// A boolean value.
pub fn is_any_file_staged() -> bool {
    exec_git(vec!["status"])
        .map(|output| {
            std::str::from_utf8(&output.stdout)
                .unwrap_or("")
                .contains("Changes to be committed:")
        })
        .expect("Failed to determine if files have been staged")
}

/// It runs `git status -s` and checks if any of the lines start with `AA`, `AU`, `DD`, `DU`, `UA`,
/// `UD`, or `UU`
///
/// Returns:
///
/// A boolean value
pub fn is_any_file_conflicting() -> bool {
    exec_git(vec!["status", "-s"])
        .map(|output| {
            std::str::from_utf8(&output.stdout)
                .unwrap_or("")
                .lines()
                .any(|l| {
                    vec!["AA", "AU", "DD", "DU", "UA", "UD", "UU"]
                        .binary_search(&&l[..2])
                        .is_ok()
                })
            // UU AA AU UA DU UD DD
        })
        .expect("Failed to determine if files are conflicting with remote repo")
}

/// It commits all staged files with the given message
///
/// Arguments:
///
/// * `message`: &str
pub fn commit_staged_files(message: &str) {
    stage_files();
    if is_any_file_staged() {
        exec_git_with_logs(vec!["commit", "-m", &message])
            .expect(format!("Failed to commit: {}", &message).as_str());
    } else {
        println!("There are no staged files. Commit has been aborted.");
    }
}

/// It fetches the latest commits from the remote repository and overwrites the local repository with
/// them
pub fn fetch() {
    exec_git_with_logs(vec!["fetch", "origin"]).expect("Failed to fetch from remote git repo");
    exec_git_with_logs(vec!["reset", "--hard", "origin/master"])
        .expect("Overwrite local with remote commits");
}

/// It executes the git push command
pub fn push() {
    exec_git_with_logs(vec!["push"]).expect("Failed to push to remote git repo");
}

/// It executes the `git push` command with the argument `--force`
pub fn force_push() {
    exec_git_with_logs(vec!["push", "--force"]).expect("Failed to force push to remote git repo");
}
