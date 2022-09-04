use inquire::formatter::OptionFormatter;
use inquire::{Confirm, Select, Text};
use std::fmt;

use crate::config::load_config;
use crate::fs::clean_working_dir;
use crate::git;
use crate::process::{get_found_mappings, init_working_dir, link_mappings, track_links};

/// Creating an enum called State with the values ActionSelection, Initialize, Clean, Refresh, Commit,
/// Fetch, Push, and Exit.
#[derive(PartialEq, Clone)]
pub enum State {
    ActionSelection,
    Initialize,
    Clean,
    Refresh,
    Commit,
    Fetch,
    Push,
    Exit,
}

/// Implementing the `Display` trait for the `State` enum.
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::ActionSelection => write!(f, "Choose what to do next"),
            State::Initialize => write!(f, "Initialize current working directory"),
            State::Clean => write!(f, "Clean items"),
            State::Refresh => write!(f, "Refresh items"),
            State::Commit => write!(f, "Commit changes"),
            State::Fetch => write!(f, "Fetch from remote"),
            State::Push => write!(f, "Update remote"),
            State::Exit => write!(f, "Exit"),
        }
    }
}

impl State {
    pub fn from(str: &str) -> State {
        match str {
            "options" => Self::ActionSelection,
            "init" => State::Initialize,
            "clean" => State::Clean,
            "refresh" => State::Refresh,
            "commit" => State::Commit,
            "fetch" => State::Fetch,
            "push" => State::Push,
            "exit" => State::Exit,
            _ => panic!("{:?} is not a valid option", &str),
        }
    }
}

/// `get_next_action` is a function that returns a next `State` enum.
/// It creates a `Select` object with a prompt and a list of options
/// fromwhich a user can select
///
/// Returns:
///
/// A State enum
fn get_next_action() -> State {
    let formatter: OptionFormatter<State> = &|a| format!("Chose to {a}");
    let actions = vec![
        State::Initialize,
        State::Clean,
        State::Refresh,
        State::Commit,
        State::Fetch,
        State::Push,
        State::Exit,
    ];
    Select::new("What would you like to do?", actions)
        .with_formatter(formatter)
        .prompt()
        .expect("Failed to capture selection(s)")
}

/// It loads the config file, finds all the mappings, and links them to working directory
fn try_refresh() {
    clean_working_dir();
    let config = load_config("config.cmf")
        .expect("Failed to load config file. Try initializing first.");
    let links = link_mappings(&get_found_mappings(&config));
    track_links(&links);
}

/// It prompts the user for a commit message, and if the user enters a message, it commits the staged
/// files with that message
fn try_commit() {
    match Text::new("Write your commit message here: ").prompt() {
        Ok(message) => git::commit_staged_files(message.as_str()),
        Err(e) => println!("Failed to get commit message: {:?}", e),
    }
}

/// Pushes chages to remote repo.
/// If there are any files that have been updated both locally and remotely, ask the user if they want
/// to force push the local changes to the remote repo
fn try_push() {
    if git::is_any_file_conflicting() {
        let force = Confirm::new("Some files have been updated both locally and remotely. Replace remote repo with local changes?")
            .with_default(false)
            .with_help_message("This will ensure your remote copy matches the local setup.")
            .prompt();
        if let Ok(true) = force {
            git::force_push();
        }
    } else {
        git::push();
    }
}

/// It takes a `Config` and a current `State` executes it and returns the next `State`
///
/// Arguments:
///
/// * `state`: &State - This is the current state of the program.
///
/// Returns:
///
/// A State enum
pub fn run_once(state: State) -> State {
    match state {
        State::Initialize => init_working_dir(),
        State::Clean => clean_working_dir(),
        State::Refresh => try_refresh(),
        State::Commit => try_commit(),
        State::Fetch => git::fetch(),
        State::Push => try_push(),
        State::ActionSelection => return get_next_action(),
        State::Exit => return State::Exit,
    }
    return State::ActionSelection;
}

/// Run the application on loop until the user exits.
///
/// Arguments:
///
/// * `state`: The current state of the program.
pub fn run(mut state: State) {
    while state != State::Exit {
        state = run_once(state);
    }
}
