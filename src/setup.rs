use crate::utils::get_storage_folder_path;
use colored::Colorize;
use std::fs;
use std::process::{self, Command, Stdio};

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

pub fn setup() {
    check_if_git_is_installed();

    let storage_folder_path = get_storage_folder_path();

    if storage_folder_path.is_dir() {
        return;
    }

    if fs::create_dir_all(&storage_folder_path).is_err() {
        eprintln!(
            "{}",
            "An error ocurred while trying to create the storage folder".red()
        );

        process::exit(1);
    }

    let storage_path = fs::canonicalize(&storage_folder_path).unwrap_or_else(|err| {
        let err_message = format!(
            "The following error ocurred when trying to canonicalize the storage path: {err}"
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
                format!("The following error has ocurred while trying to git init: {err}");

            eprintln!("{}", err_message.red());

            process::exit(1);
        });
}
