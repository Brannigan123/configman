pub fn exec_git(arg: Vec<&str>) -> Result<Output, Error> {
    Command::new("git").args(arg).output()
}