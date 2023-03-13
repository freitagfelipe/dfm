use super::Command;
use crate::error::{CommandError, ExecutionError};
use crate::git::GitCommandExecuterBuilder;
use crate::utils;
use clap::Args;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File does not exist in the repository")]
    FileDoesNotExists,
    #[error("You need to set a remote repository before use DFM")]
    SetRemoteRepository,
    #[error("No internet connection")]
    NoInternetConnection,
}

impl From<Error> for CommandError {
    fn from(err: Error) -> Self {
        CommandError::Usage(err.to_string())
    }
}

/// Removes a file from the remote repository
#[derive(Debug, Args)]
pub struct Remove {
    /// File name
    name: String,
}

impl Command for Remove {
    fn execute(self) -> Result<String, CommandError> {
        if online::check(None).is_err() {
            return Err(Error::NoInternetConnection.into());
        }

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

        if !utils::check_if_file_exists(&git_storage_folder_path, &self.name) {
            return Err(Error::FileDoesNotExists.into());
        }

        if let Err(err) = fs::remove_file(git_storage_folder_path.join(&self.name)) {
            return Err(ExecutionError::RemoveFile(err.to_string()).into());
        }

        GitCommandExecuterBuilder::new(&git_storage_folder_path)
            .run_commit(&format!("Remove {}", self.name))
            .build()
            .run()?;

        Ok("Successfully removed the file and synchronized the local repository with the remote repository".to_string())
    }
}
