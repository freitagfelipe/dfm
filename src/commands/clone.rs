use super::Command;
use crate::error::{CommandError, ExecutionError};
use crate::utils;
use clap::Args;
use std::{env, fs};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File does not exist in the repository")]
    FileDoesNotExists,
}

impl From<Error> for CommandError {
    fn from(err: Error) -> Self {
        CommandError::Usage(err.to_string())
    }
}

/// Clones a file from the repository to your current directory
#[derive(Debug, Args)]
pub struct Clone {
    /// File name
    name: String,
}

impl Command for Clone {
    fn execute(self) -> Result<String, CommandError> {
        let git_storage_folder_path = match utils::get_dfm_folder_path() {
            Ok(path) => path,
            Err(err) => {
                return Err(ExecutionError::GetStorageFolderPath(err.to_string()).into());
            }
        };

        let git_storage_folder_path = match git_storage_folder_path.canonicalize() {
            Ok(path) => path,
            Err(err) => {
                return Err(ExecutionError::CanonicalizePath(err.to_string()).into());
            }
        };

        if !utils::check_if_file_exists(&git_storage_folder_path, &self.name) {
            return Err(Error::FileDoesNotExists.into());
        }

        let current_dir = match env::current_dir() {
            Ok(path) => path,
            Err(err) => {
                return Err(ExecutionError::GetCurrentDir(err.to_string()).into());
            }
        };

        if let Err(err) = fs::copy(
            git_storage_folder_path.join(&self.name),
            current_dir.join(&self.name),
        ) {
            return Err(ExecutionError::CopyFile(err.to_string()).into());
        }

        Ok("Successfully cloned the file to your current directory".to_string())
    }
}
