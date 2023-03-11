use super::Command;
use crate::error::{CommandError, ExecutionError};
use crate::git::GitCommandExecuterBuilder;
use crate::utils;
use clap::{Args, Subcommand};
use regex::Regex;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "Remote repository already added if you want to change that you need to reset your DFM"
    )]
    AlreadyAdded,
    #[error("Remote repository not setted yet")]
    NotSetted,
    #[error("Not a ssh link")]
    NotSSH,
}

impl From<Error> for CommandError {
    fn from(err: Error) -> Self {
        CommandError::Usage(err.to_string())
    }
}

/// Manages the remote repository
#[derive(Debug, Args)]
pub struct Remote {
    #[command(subcommand)]
    subcommands: Subcommands,
}

/// Sets the remote repository to the passed link (you can only do that once without resetting DFM)
#[derive(Debug, Args)]
pub struct Set {
    /// Remote repository link
    link: String,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    /// Shows the current remote repository
    Show,
    /// Sets (if not setted yet) the remote repository
    Set(Set),
}

fn set_remote_link(
    storage_folder_path: &Path,
    git_storage_folder_path: &Path,
    link: &str,
) -> Result<String, CommandError> {
    if storage_folder_path.join("remote.txt").exists() {
        return Err(Error::AlreadyAdded.into());
    }

    let regex = match Regex::new(r"git@git((hub)|(lab))\.com:\S*/\S*\.git") {
        Ok(regex) => regex,
        Err(err) => {
            return Err(ExecutionError::Regex(err.to_string()).into());
        }
    };

    if !regex.is_match(link) {
        return Err(Error::NotSSH.into());
    }

    let mut file = match File::create(storage_folder_path.join("remote.txt")) {
        Ok(file) => file,
        Err(err) => {
            return Err(ExecutionError::CreateFile(err.to_string()).into());
        }
    };

    if let Err(err) = file.write_all(link.as_bytes()) {
        return Err(ExecutionError::WriteToFile(err.to_string()).into());
    }

    GitCommandExecuterBuilder::new(git_storage_folder_path)
        .run_remote_add(link)
        .run_pull()
        .build()
        .run()?;

    Ok("Successfully setted the remote repository and synchronized the local repository with the remote repository".to_string())
}

fn show_remote_link(storage_folder_path: &Path) -> Result<String, CommandError> {
    if !storage_folder_path.join("remote.txt").exists() {
        return Err(Error::NotSetted.into());
    }

    let mut file = match File::open(storage_folder_path.join("remote.txt")) {
        Ok(file) => file,
        Err(err) => {
            return Err(ExecutionError::OpenFile(err.to_string()).into());
        }
    };

    let mut content = Vec::new();

    if let Err(err) = file.read_to_end(&mut content) {
        return Err(ExecutionError::ReadFile(err.to_string()).into());
    }

    let Ok(content) = String::from_utf8(content) else {
        return Err(ExecutionError::InvalidUTF8("Convert Vec<u8> to String").into());
    };

    Ok(content)
}

impl Command for Remote {
    fn execute(self) -> Result<String, CommandError> {
        let storage_folder_path = match utils::get_storage_folder_path() {
            Ok(path) => path,
            Err(err) => {
                return Err(ExecutionError::GetStorageFolderPath(err.to_string()).into());
            }
        };

        let storage_folder_path = match storage_folder_path.canonicalize() {
            Ok(path) => path,
            Err(err) => {
                return Err(ExecutionError::CanonicalizePath(err.to_string()).into());
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
