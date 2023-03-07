use super::Command;
use crate::utils;
use clap::Args;
use std::path::Path;
use std::process::{Command as Cmd, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("You do not have setted a remote repository yet")]
    NoRemoteRepository,
    #[error("{0}")]
    GetStorageFolderPath(String),
    #[error("Something wrong happened: {0}, when trying to: {1}")]
    Unknown(String, &'static str),
}

/// Synchronizes the local repository with the remote repository
#[derive(Debug, Args)]
pub struct Push;

fn execute_git_commands(storage_folder_path: &Path) -> Result<(), Error> {
    let mut handler = match Cmd::new("git")
        .args(["push", "origin", "main"])
        .current_dir(storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(handler) => handler,
        Err(err) => {
            return Err(Error::Unknown(
                err.to_string(),
                "execute git push origin main",
            ))
        }
    };

    if let Err(err) = handler.wait() {
        return Err(Error::Unknown(err.to_string(), "wait git push origin main"));
    }

    Ok(())
}

impl Command for Push {
    type Error = Error;

    fn execute(self) -> Result<String, Self::Error> {
        let storage_folder_path = match utils::get_storage_folder_path() {
            Ok(path) => path,
            Err(err) => return Err(Error::GetStorageFolderPath(err.to_string()))
        };

        let storage_folder_path = match storage_folder_path.canonicalize() {
            Ok(path) => path,
            Err(err) => {
                return Err(Error::Unknown(
                    err.to_string(),
                    "canonicalize the storage folder path",
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
