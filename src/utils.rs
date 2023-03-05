use colored::Colorize;
use std::env;
use std::path::{Path, PathBuf};
use std::process;

pub fn get_storage_folder_path() -> PathBuf {
    if cfg!(any(target_os = "linux", target_os = "macos")) {
        let home_path = env::var("HOME").unwrap_or_else(|err| {
            let err_message = format!(
                "The following error ocurred while trying to get the HOME enviroment variable: {err}"
            );

            eprintln!("{}", err_message.red());

            process::exit(2);
        });

        PathBuf::from(format!("{home_path}/.config/dotfiles"))
    } else {
        let home_path = env::var("APPDATA").unwrap_or_else(|err| {
            let err_message = format!(
                "The following error ocurred while trying to get the APPDATA enviroment variable: {err}"
            );

            eprintln!("{}", err_message.red());

            process::exit(2);
        });

        PathBuf::from(format!("{home_path}\\dotfiles"))
    }
}

pub fn check_if_file_exists(folder: &Path, file_name: &str) -> bool {
    folder.join(file_name).exists()
}
