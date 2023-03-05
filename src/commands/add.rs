use super::Command;
use crate::{setup::get_storage_folder_path, utils::check_if_file_exists};
use clap::Args;
use std::path::Path;
use std::process::{Command as Cmd, Stdio};
use std::{env, fs};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File already added")]
    FileAlreadyAdded,
    #[error("File does not exists in the current folder")]
    FileDoesNotExists,
    #[error("Something wrong happened: {0}")]
    Unknown(String),
}

/// Add a file to the repository
#[derive(Debug, Args)]
pub struct Add {
    name: String,
}

fn execute_git_commands(storage_folder: &Path, file_name: &str) -> Result<(), Error> {
    if let Err(err) = Cmd::new("git")
        .args(["add", "."])
        .current_dir(storage_folder)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        return Err(Error::Unknown(err.to_string()));
    }

    if let Err(err) = Cmd::new("git")
        .args(["commit", "-m", &format!("\"Add {file_name}\"")])
        .current_dir(storage_folder)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        return Err(Error::Unknown(err.to_string()));
    };

    Ok(())
}

impl Command for Add {
    type Error = Error;

    fn execute(self) -> Result<&'static str, Self::Error> {
        let storage_folder_path = get_storage_folder_path();

        let current_dir = match env::current_dir() {
            Ok(path) => path,
            Err(err) => {
                return Err(Error::Unknown(err.to_string()));
            }
        };

        if !check_if_file_exists(&current_dir, &self.name) {
            return Err(Error::FileDoesNotExists);
        }

        if check_if_file_exists(&storage_folder_path, &self.name) {
            return Err(Error::FileAlreadyAdded);
        }

        if fs::copy(
            current_dir.join(&self.name),
            storage_folder_path.join(&self.name),
        )
        .is_err()
        {
            return Err(Error::Unknown(
                "An error ocurred while trying to copy the file".to_string(),
            ));
        }

        let storage_folder_path = match fs::canonicalize(&storage_folder_path) {
            Ok(path) => path,
            Err(err) => return Err(Error::Unknown(err.to_string())),
        };

        execute_git_commands(&storage_folder_path, &self.name)?;

        Ok("Successfully added the file")
    }
}
