use colored::Colorize;
use std::process::{self, Command, Stdio};

pub fn setup() {
    if Command::new("git")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("Some error ocurred while checking if git is installed")
        .code()
        .expect("Process terminated by a signal")
        != 1
    {
        eprintln!("{}", "You need to have git installed to use DFM".red());

        process::exit(2);
    }
}
