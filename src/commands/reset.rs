use super::Command;
use crate::setup;
use crate::utils;
use clap::Args;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    SetupRelated(String),
    #[error("{0}")]
    GetStorageFolderPath(String),
    #[error("You need to set a remote repository before use DFM")]
    SetRemoteRepository,
    #[error("Something wrong happened: {0}, when trying to: {1}")]
    Unknown(String, &'static str),
}

/// Resets your DFM to the initial state (be careful using that)
#[derive(Debug, Args)]
pub struct Reset;

impl Command for Reset {
    type Error = Error;

    fn execute(self) -> Result<String, Self::Error> {
        let git_storage_folder_path = match utils::get_git_storage_folder_path() {
            Ok(path) => path,
            Err(err) => return Err(Error::GetStorageFolderPath(err.to_string())),
        };

        let git_storage_folder_path = match git_storage_folder_path.canonicalize() {
            Ok(path) => path,
            Err(err) => {
                return Err(Error::Unknown(
                    err.to_string(),
                    "canonicalize the storage folder path",
                ))
            }
        };

        if utils::check_if_remote_link_is_added().is_err() {
            return Err(Error::SetRemoteRepository);
        }

        if let Err(err) = fs::remove_dir_all(&git_storage_folder_path) {
            return Err(Error::Unknown(err.to_string(), "remove the storage folder"));
        }

        if let Err(err) = fs::create_dir_all(&git_storage_folder_path) {
            return Err(Error::Unknown(err.to_string(), "create the storage folder"));
        }

        if let Err(err) = setup::execute_git_commands(&git_storage_folder_path) {
            return Err(Error::SetupRelated(err.to_string()));
        }

        Ok("Successfully reseted the DFM to the initial state".to_string())
    }
}
