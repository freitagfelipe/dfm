use super::Command;
use crate::utils::{check_if_file_exists, get_storage_folder_path};
use clap::Args;
use std::fs;
use std::path::Path;
use std::process::{Command as Cmd, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File does not exists in the repository")]
    FileDoesNotExists,
    #[error("Something wrong happened: {0}")]
    Unknown(String),
}

/// Removes a file of the repository
#[derive(Debug, Args)]
pub struct Remove {
    /// File name
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
        .args(["commit", "-m", &format!("\"Remove {file_name}\"")])
        .current_dir(storage_folder)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        return Err(Error::Unknown(err.to_string()));
    };

    Ok(())
}

impl Command for Remove {
    type Error = Error;

    fn execute(self) -> Result<&'static str, Self::Error> {
        let storage_folder_path = match get_storage_folder_path().canonicalize() {
            Ok(path) => path,
            Err(err) => return Err(Error::Unknown(err.to_string())),
        };

        if !check_if_file_exists(&storage_folder_path, &self.name) {
            return Err(Error::FileDoesNotExists);
        }

        if let Err(err) = fs::remove_file(storage_folder_path.join(&self.name)) {
            return Err(Error::Unknown(err.to_string()));
        }

        execute_git_commands(&storage_folder_path, &self.name)?;

        Ok("Successfully removed the file")
    }
}
