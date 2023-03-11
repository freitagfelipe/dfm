use super::Command;
use crate::error::{CommandError, ExecutionError};
use crate::git::GitCommandExecuterBuilder;
use crate::utils;
use clap::Args;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
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

/// Syncs with the remote repository
#[derive(Debug, Args)]
pub struct Sync;

impl Command for Sync {
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

        GitCommandExecuterBuilder::new(&git_storage_folder_path)
            .run_pull()
            .build()
            .run()?;

        Ok("Finished the synchronization with the remote repository".to_string())
    }
}
