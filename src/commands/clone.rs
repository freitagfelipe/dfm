use super::Command;
use crate::utils;
use clap::Args;
use std::{env, fs};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File does not exist in the repository")]
    FileDoesNotExists,
    #[error("{0}")]
    GetStorageFolderPath(String),
    #[error("You need to set a remote repository before use DFM")]
    Unknown(String, &'static str),
}

/// Clones a file from the repository to your current directory
#[derive(Debug, Args)]
pub struct Clone {
    /// File name
    name: String,
}

impl Command for Clone {
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

        if !utils::check_if_file_exists(&storage_folder_path, &self.name) {
            return Err(Error::FileDoesNotExists);
        }

        let current_dir = match env::current_dir() {
            Ok(path) => path,
            Err(err) => {
                return Err(Error::Unknown(err.to_string(), "get the current dir"));
            }
        };

        if let Err(err) = fs::copy(
            storage_folder_path.join(&self.name),
            current_dir.join(&self.name),
        ) {
            return Err(Error::Unknown(err.to_string(), "copy the file"));
        }

        Ok("Successfully cloned the file to your current directory".to_string())
    }
}
