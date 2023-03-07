use super::Command;
use crate::git;
use crate::utils;
use clap::{Args, Subcommand};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Remote repository already added if you want to change you need to reset DFM")]
    AlreadyAdded,
    #[error("Remote repository not setted yet")]
    NotSetted,
    #[error("{0}")]
    GetStorageFolderPath(String),
    #[error("{0}")]
    GitCommand(String),
    #[error("Something wrong happened: {0}, when trying to: {1}")]
    Unknown(String, &'static str),
}

/// Manages the remote repository.
#[derive(Debug, Args)]
pub struct Remote {
    #[command(subcommand)]
    subcommands: Subcommands,
}

/// Sets the remote repository to the passed link. You can only do that once without resetting DFM.
#[derive(Debug, Args)]
pub struct Set {
    /// Link to the remote repository
    link: String,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    /// Shows the current remote repository
    Show,
    /// Sets (if not setted yet) the remote repository
    Set(Set),
}

fn execute_git_command(git_storage_folder_path: &Path, link: &str) -> Result<(), Error> {
    if let Err(err) = git::ExecuterBuilder::new(git_storage_folder_path)
        .run_remote_add(link)
        .run_pull()
        .build()
        .run()
    {
        return Err(Error::GitCommand(err.to_string()));
    }

    Ok(())
}

fn set_remote_link(
    storage_folder_path: &Path,
    git_storage_folder_path: &Path,
    link: &str,
) -> Result<String, Error> {
    if storage_folder_path.join("remote.txt").exists() {
        return Err(Error::AlreadyAdded);
    }

    let mut file = match File::create(storage_folder_path.join("remote.txt")) {
        Ok(file) => file,
        Err(err) => return Err(Error::Unknown(err.to_string(), "create a file")),
    };

    if let Err(err) = file.write_all(link.as_bytes()) {
        return Err(Error::Unknown(err.to_string(), "write to a file"));
    }

    execute_git_command(git_storage_folder_path, link)?;

    Ok("Successfully setted the remote repository and synchronized".to_string())
}

fn show_remote_link(storage_folder_path: &Path) -> Result<String, Error> {
    if !storage_folder_path.join("remote.txt").exists() {
        return Err(Error::NotSetted);
    }

    let mut file = match File::open(storage_folder_path.join("remote.txt")) {
        Ok(file) => file,
        Err(err) => return Err(Error::Unknown(err.to_string(), "open a file")),
    };

    let mut content = Vec::new();

    if let Err(err) = file.read_to_end(&mut content) {
        return Err(Error::Unknown(err.to_string(), "read a file"));
    }

    let Ok(content) = String::from_utf8(content) else {
        return Err(Error::Unknown("Invalid UTF-8".to_string(), "convert vec<u8> to String"));
    };

    Ok(content)
}

impl Command for Remote {
    type Error = Error;

    fn execute(self) -> Result<String, Self::Error> {
        let storage_folder_path = match utils::get_storage_folder_path() {
            Ok(path) => path,
            Err(err) => return Err(Error::GetStorageFolderPath(err.to_string())),
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

        match self.subcommands {
            Subcommands::Show => show_remote_link(&storage_folder_path),
            Subcommands::Set(set) => set_remote_link(
                &storage_folder_path,
                &storage_folder_path.join("dotfiles"),
                &set.link,
            ),
        }
    }
}
