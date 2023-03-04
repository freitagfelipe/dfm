use super::Command;
use crate::{setup::get_storage_folder_path, utils::check_if_file_exists};
use clap::Args;
use colored::Colorize;
use std::path::Path;
use std::process::{self, Command as Cmd, Stdio};
use std::{env, fs};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File already added")]
    FileAlreadyAdded,
    #[error("File does not exists in the current folder")]
    FileDoesNotExists,
}

/// Add a file to the repository
#[derive(Debug, Args)]
pub struct Add {
    name: String,
}

fn execute_git_commands(storage_folder: &Path, file_name: &str) {
    Cmd::new("git")
        .args(["add", "."])
        .current_dir(storage_folder)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap_or_else(|err| {
            let err_message =
                format!("The following error ocurred when trying to git add: {err}");

            eprintln!("{}", err_message.red());

            process::exit(1);
        });

    Cmd::new("git")
        .args(["commit", "-m", &format!("\"Add {file_name}\"")])
        .current_dir(storage_folder)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap_or_else(|err| {
            let err_message =
                format!("The following error ocurred when trying to git commit: {err}");

            eprintln!("{}", err_message.red());

            process::exit(1);
        });
}

impl Command for Add {
    type Error = Error;


    fn execute(self) -> Result<&'static str, Self::Error> {
        let storage_folder = get_storage_folder_path();

        let current_dir = match env::current_dir() {
            Ok(path) => path,
            Err(err) => {
                let err_message = format!(
                    "The following error ocurred when trying to get the current directory {err}"
                );

                eprintln!("{}", err_message.red());

                process::exit(1);
            }
        };

        if !check_if_file_exists(&current_dir, &self.name) {
            return Err(Error::FileDoesNotExists);
        }

        if check_if_file_exists(&storage_folder, &self.name) {
            return Err(Error::FileAlreadyAdded);
        }

        if fs::copy(
            current_dir.join(&self.name),
            storage_folder.join(&self.name),
        )
        .is_err()
        {
            eprintln!("{}", "An error ocurred while trying to copy the file".red());

            process::exit(1);
        }

        let storage_folder = fs::canonicalize(&storage_folder).unwrap_or_else(|err| {
            let err_message = format!(
                "The following error ocurred when trying to canonicalize the storage path: {err}"
            );

            eprintln!("{}", err_message.red());

            process::exit(1);
        });

        execute_git_commands(&storage_folder, &self.name);

        Ok("Successfully added the file")
    }
}
