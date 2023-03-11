use super::Command;
use crate::error::{CommandError, ExecutionError};
use crate::setup;
use crate::utils;
use clap::Args;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("You need to set a remote repository before use DFM")]
    SetRemoteRepository,
}

impl From<Error> for CommandError {
    fn from(err: Error) -> Self {
        CommandError::Usage(err.to_string())
    }
}

/// Resets your DFM to the initial state (be careful using that)
#[derive(Debug, Args)]
pub struct Reset;

impl Command for Reset {
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

        if let Err(err) = fs::remove_dir_all(&git_storage_folder_path) {
            return Err(ExecutionError::RemoveStorageFolder(err.to_string()).into());
        }

        if let Err(err) = fs::create_dir_all(&git_storage_folder_path) {
            return Err(ExecutionError::CreateStorageFolder(err.to_string()).into());
        }

        setup::execute_git_commands(&git_storage_folder_path)?;

        Ok("Successfully reseted the DFM to the initial state".to_string())
    }
}
