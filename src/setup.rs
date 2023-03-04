use colored::Colorize;
use std::env;
use std::fs;
use std::path::Path;
use std::process::{self, Command, Stdio};

pub fn get_storage_folder_path() -> String {
    if cfg!(any(target_os = "linux", target_os = "macos")) {
        let home_path = env::var("HOME").unwrap_or_else(|err| {
            let err_message = format!(
                "The following error ocurred while trying to get the HOME enviroment variable: {err}"
            );

            eprintln!("{}", err_message.red());

            process::exit(2);
        });

        format!("{home_path}/.config/dotfiles")
    } else {
        let home_path = env::var("APPDATA").unwrap_or_else(|err| {
            let err_message = format!(
                "The following error ocurred while trying to get the APPDATA enviroment variable: {err}"
            );

            eprintln!("{}", err_message.red());

            process::exit(2);
        });

        format!("{home_path}\\dotfiles")
    }
}

fn check_if_git_is_installed() {
    let command_code = Command::new("git")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap_or_else(|err| {
            let err_message =
                format!("The following error ocurred while checking if git is installed: {err}");

            eprintln!("{}", err_message.red());

            process::exit(1);
        })
        .code()
        .unwrap_or_else(|| {
            eprintln!("{}", "Process terminated by a signal".red());

            process::exit(1);
        });

    if command_code != 1 {
        eprintln!("{}", "You need to have git installed to use DFM".red());

        process::exit(2);
    }
}

fn check_if_storage_dir_is_created() -> bool {
    let storage_folder_path = get_storage_folder_path();

    Path::new(&storage_folder_path).is_dir()
}

pub fn setup() {
    check_if_git_is_installed();

    if check_if_storage_dir_is_created() {
        return;
    }

    let storage_path = get_storage_folder_path();

    if fs::create_dir_all(Path::new(&storage_path)).is_err() {
        eprintln!(
            "{}",
            "An error ocurred while trying to create the storage folder".red()
        );

        process::exit(1);
    }

    let storage_path = fs::canonicalize(Path::new(&storage_path)).unwrap_or_else(|err| {
        let err_message = format!(
            "The following error ocurred when trying to canonicalize the storage path {err}"
        );

        eprintln!("{}", err_message.red());

        process::exit(1);
    });

    Command::new("git")
        .arg("init")
        .current_dir(storage_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap_or_else(|err| {
            let err_message =
                format!("The following error has ocurred while trying to git init {err}");

            eprintln!("{}", err_message.red());

            process::exit(1);
        });
}
