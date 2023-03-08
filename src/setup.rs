use crate::git::ExecuterBuilder;
use crate::utils;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("You need to have git installed to use DFM")]
    NeedGit,
    #[error("{0}")]
    GetStorageFolderPath(String),
    #[error("{0}")]
    GitCommand(String),
    #[error("Something wrong happened: {0}, when trying to: {1}")]
    Unknown(String, &'static str),
}

fn check_if_git_is_installed() -> Result<(), Error> {
    let status = match Command::new("git")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Ok(status) => status,
        Err(err) => return Err(Error::Unknown(err.to_string(), "execute git")),
    };

    let Some(command_code) = status.code() else {
        return Err(Error::Unknown("process terminated by signal".to_string(), "get git status code"));
    };

    if command_code != 1 {
        return Err(Error::NeedGit);
    }

    Ok(())
}

pub fn execute_git_commands(git_storage_folder_path: &Path) -> Result<(), Error> {
    if let Err(err) = ExecuterBuilder::new(git_storage_folder_path)
        .run_init()
        .build()
        .run()
    {
        return Err(Error::GitCommand(err.to_string()));
    }

    Ok(())
}

pub fn setup() -> Result<(), Error> {
    check_if_git_is_installed()?;

    let git_storage_folder_path = match utils::get_git_storage_folder_path() {
        Ok(path) => path,
        Err(err) => return Err(Error::GetStorageFolderPath(err.to_string())),
    };

    if git_storage_folder_path.is_dir() {
        return Ok(());
    }

    if let Err(err) = fs::create_dir_all(&git_storage_folder_path) {
        return Err(Error::Unknown(err.to_string(), "create the storage folder"));
    }

    let storage_folder_path = match git_storage_folder_path.canonicalize() {
        Ok(path) => path,
        Err(err) => {
            return Err(Error::Unknown(
                err.to_string(),
                "canonicalize the storage folder path",
            ));
        }
    };

    execute_git_commands(&storage_folder_path)?;

    Ok(())
}
