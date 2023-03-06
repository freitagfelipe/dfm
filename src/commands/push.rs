use super::Command;
use crate::utils::get_storage_folder_path;
use clap::Args;
use std::path::Path;
use std::process::{Command as Cmd, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("You do not have setted a remote repository yet")]
    NoRemoteRepository,
    #[error("Something wrong happened: {0}, when trying to: {1}")]
    Unknown(String, &'static str),
}

/// Synchronizes the local repository with the remote repository
#[derive(Debug, Args)]
pub struct Push;

fn execute_git_commands(storage_folder: &Path) -> Result<(), Error> {
    if let Err(err) = Cmd::new("git")
        .args(["push", "origin", "main"])
        .current_dir(storage_folder)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        return Err(Error::Unknown(
            err.to_string(),
            "execute git push origin main",
        ));
    }

    Ok(())
}

impl Command for Push {
    type Error = Error;

    fn execute(self) -> Result<String, Self::Error> {
        let storage_folder_path = match get_storage_folder_path().canonicalize() {
            Ok(path) => path,
            Err(err) => {
                return Err(Error::Unknown(
                    err.to_string(),
                    "canonicalize the storage path",
                ))
            }
        };

        if !storage_folder_path.join("remote.txt").exists() {
            return Err(Error::NoRemoteRepository);
        }

        execute_git_commands(&storage_folder_path)?;

        Ok("Successfully pushed your changes to the remote repository".to_string())
    }
}
