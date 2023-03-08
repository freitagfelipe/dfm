use super::Command;
use crate::git::ExecuterBuilder;
use crate::utils::{self, CommonError};
use clap::Args;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File does not exist in the repository")]
    FileDoesNotExists,
    #[error("{0}")]
    Common(CommonError),
}

/// Removes a file from the remote repository
#[derive(Debug, Args)]
pub struct Remove {
    /// File name
    name: String,
}

fn execute_git_commands(git_storage_folder_path: &Path, file_name: &str) -> Result<(), Error> {
    if let Err(err) = ExecuterBuilder::new(git_storage_folder_path)
        .run_commit(&format!("Remove {file_name}"))
        .build()
        .run()
    {
        return Err(Error::Common(CommonError::GitCommand(err.to_string())));
    }

    Ok(())
}

impl Command for Remove {
    type Error = Error;

    fn execute(self) -> Result<String, Self::Error> {
        let git_storage_folder_path = match utils::get_git_storage_folder_path() {
            Ok(path) => path,
            Err(err) => {
                return Err(Error::Common(CommonError::GetStorageFolderPath(
                    err.to_string(),
                )))
            }
        };

        let storage_folder_path = match git_storage_folder_path.canonicalize() {
            Ok(path) => path,
            Err(err) => {
                return Err(Error::Common(CommonError::Unknown(
                    err.to_string(),
                    "canonicalize the storage folder path",
                )))
            }
        };

        if utils::check_if_remote_link_is_added().is_err() {
            return Err(Error::Common(CommonError::SetRemoteRepository));
        }

        if !utils::check_if_file_exists(&storage_folder_path, &self.name) {
            return Err(Error::FileDoesNotExists);
        }

        if let Err(err) = fs::remove_file(storage_folder_path.join(&self.name)) {
            return Err(Error::Common(CommonError::Unknown(
                err.to_string(),
                "remove a file",
            )));
        }

        execute_git_commands(&storage_folder_path, &self.name)?;

        Ok("Successfully removed the file and synchronized the local repository with the remote repository".to_string())
    }
}
