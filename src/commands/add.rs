use super::Command;
use crate::git;
use crate::utils;
use clap::Args;
use std::env;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File already added to the DFM repository")]
    FileAlreadyAdded,
    #[error("File does not exists in your current directory")]
    FileDoesNotExists,
    #[error("You can just add files to the repository")]
    NotAFile,
    #[error("{0}")]
    GetStorageFolderPath(String),
    #[error("{0}")]
    GitCommand(String),
    #[error("Something wrong happened: {0}, when trying to: {1}")]
    Unknown(String, &'static str),
}

/// Adds a file to the repository
#[derive(Debug, Args)]
pub struct Add {
    /// File name
    name: String,
}

fn execute_git_commands(storage_folder_path: &Path, file_name: &str) -> Result<(), Error> {
    let mut handler = match git::add_all(storage_folder_path) {
        Ok(handler) => handler,
        Err(err) => return Err(Error::GitCommand(err.to_string())),
    };

    if let Err(err) = handler.wait() {
        return Err(Error::Unknown(err.to_string(), "wait git add . finish"));
    }

    let mut handler = match git::commit(storage_folder_path, &format!("Add {file_name}")) {
        Ok(handler) => handler,
        Err(err) => return Err(Error::GitCommand(err.to_string())),
    };

    if let Err(err) = handler.wait() {
        return Err(Error::Unknown(err.to_string(), "wait git add . finish"));
    }

    let mut handler = match git::push(storage_folder_path) {
        Ok(handler) => handler,
        Err(err) => return Err(Error::GitCommand(err.to_string())),
    };

    if let Err(err) = handler.wait() {
        return Err(Error::Unknown(err.to_string(), "wait git push origin main"));
    }

    Ok(())
}

impl Command for Add {
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

        let current_dir = match env::current_dir() {
            Ok(path) => path,
            Err(err) => {
                return Err(Error::Unknown(err.to_string(), "get the current dir"));
            }
        };

        if !utils::check_if_file_exists(&current_dir, &self.name) {
            return Err(Error::FileDoesNotExists);
        }

        if !current_dir.join(&self.name).is_file() {
            return Err(Error::NotAFile);
        }

        if utils::check_if_file_exists(&storage_folder_path, &self.name) {
            return Err(Error::FileAlreadyAdded);
        }

        if let Err(err) = fs::copy(
            current_dir.join(&self.name),
            storage_folder_path.join(&self.name),
        ) {
            return Err(Error::Unknown(err.to_string(), "copy the file"));
        }

        execute_git_commands(&storage_folder_path, &self.name)?;

        Ok("Successfully added the file".to_string())
    }
}
