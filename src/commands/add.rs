use super::Command;
use crate::error::{CommandError, ExecutionError};
use crate::git::ExecuterBuilder;
use crate::utils;
use clap::Args;
use std::env;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File already added to the remote repository")]
    FileAlreadyAdded,
    #[error("File does not exist in your current directory")]
    FileDoesNotExists,
    #[error("You can just add files to the repository")]
    NotAFile,
    #[error("You need to set a remote repository before use DFM")]
    SetRemoteRepository,
}

impl From<Error> for CommandError {
    fn from(err: Error) -> Self {
        CommandError::Usage(err.to_string())
    }
}

/// Adds a file to the remote repository
#[derive(Debug, Args)]
pub struct Add {
    /// File name
    name: String,
}

impl Command for Add {
    fn execute(self) -> Result<String, CommandError> {
        let git_storage_folder_path = match utils::get_git_storage_folder_path() {
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

        if utils::check_if_remote_link_is_added().is_err() {
            return Err(Error::SetRemoteRepository.into());
        }

        let current_dir = match env::current_dir() {
            Ok(path) => path,
            Err(err) => {
                return Err(ExecutionError::GetCurrentDir(err.to_string()).into());
            }
        };

        if !utils::check_if_file_exists(&current_dir, &self.name) {
            return Err(Error::FileDoesNotExists.into());
        }

        if !current_dir.join(&self.name).is_file() {
            return Err(Error::NotAFile.into());
        }

        if utils::check_if_file_exists(&git_storage_folder_path, &self.name) {
            return Err(Error::FileAlreadyAdded.into());
        }

        if let Err(err) = fs::copy(
            current_dir.join(&self.name),
            git_storage_folder_path.join(&self.name),
        ) {
            return Err(ExecutionError::CopyFile(err.to_string()).into());
        }

        ExecuterBuilder::new(&git_storage_folder_path)
            .run_commit(&format!("Add {}", self.name))
            .build()
            .run()?;

        Ok("Successfully added the file and synchronized the local repository with the remote repository".to_string())
    }
}
