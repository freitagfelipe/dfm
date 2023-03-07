use crate::git;
use crate::utils;
use std::fs::{self, File};
use std::io::Write;
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

pub fn create_git_ignore(storage_folder_path: &Path) -> Result<(), Error> {
    let mut file = match File::create(storage_folder_path.join(".gitignore")) {
        Ok(file) => file,
        Err(err) => return Err(Error::Unknown(err.to_string(), "create a .gitignore file")),
    };

    if let Err(err) = file.write_all(b"remote.txt") {
        return Err(Error::Unknown(
            err.to_string(),
            "write to the .gitignore file",
        ));
    };

    Ok(())
}

pub fn execute_git_commands(storage_folder_path: &Path) -> Result<(), Error> {
    if let Err(err) = git::ExecuterBuilder::new(storage_folder_path)
        .run_init()
        .run_commit("Add .gitignore")
        .build()
        .run()
    {
        return Err(Error::GitCommand(err.to_string()));
    }

    Ok(())
}

pub fn setup() -> Result<(), Error> {
    check_if_git_is_installed()?;

    let storage_folder_path = match utils::get_storage_folder_path() {
        Ok(path) => path,
        Err(err) => return Err(Error::GetStorageFolderPath(err.to_string())),
    };

    if storage_folder_path.is_dir() {
        return Ok(());
    }

    if let Err(err) = fs::create_dir_all(&storage_folder_path) {
        return Err(Error::Unknown(err.to_string(), "create the storage folder"));
    }

    let storage_folder_path = match storage_folder_path.canonicalize() {
        Ok(path) => path,
        Err(err) => {
            return Err(Error::Unknown(
                err.to_string(),
                "canonicalize the storage folder path",
            ));
        }
    };

    create_git_ignore(&storage_folder_path)?;

    execute_git_commands(&storage_folder_path)?;

    Ok(())
}
