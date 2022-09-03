mod config;
mod fs;
mod git;
mod menu;
mod process;

use menu::State;
use std::env;

/// It loads the config file, then if there are no arguments, it runs the menu in a loop, otherwise it
/// runs the menu once and then exits
fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 || args[0] == "-i" {
        let state = State::ActionSelection;
        menu::run(state);
    } else if args.len() == 2 {
        let state = State::from(args[1].as_str());
        menu::run_once(state);
    } else {
        println!("Failed to understand passed arguments");
    }
}
