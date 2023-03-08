use super::Command;
use crate::git::ExecuterBuilder;
use crate::utils::{self, CommonError};
use clap::Args;
use std::env;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File already added to the remote repository")]
    FileAlreadyAdded,
    #[error("File does not exist in your current directory")]
    FileDoesNotExists,
    #[error("You can just add files to the repository")]
    NotAFile,
    #[error("{0}")]
    Common(CommonError),
}

/// Adds a file to the remote repository
#[derive(Debug, Args)]
pub struct Add {
    /// File name
    name: String,
}

fn execute_git_commands(git_storage_folder_path: &Path, file_name: &str) -> Result<(), Error> {
    if let Err(err) = ExecuterBuilder::new(git_storage_folder_path)
        .run_commit(&format!("Add {file_name}"))
        .build()
        .run()
    {
        return Err(Error::Common(CommonError::GitCommand(err.to_string())));
    }

    Ok(())
}

impl Command for Add {
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

        let git_storage_folder_path = match git_storage_folder_path.canonicalize() {
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

        let current_dir = match env::current_dir() {
            Ok(path) => path,
            Err(err) => {
                return Err(Error::Common(CommonError::Unknown(
                    err.to_string(),
                    "get the current dir",
                )));
            }
        };

        if !utils::check_if_file_exists(&current_dir, &self.name) {
            return Err(Error::FileDoesNotExists);
        }

        if !current_dir.join(&self.name).is_file() {
            return Err(Error::NotAFile);
        }

        if utils::check_if_file_exists(&git_storage_folder_path, &self.name) {
            return Err(Error::FileAlreadyAdded);
        }

        if let Err(err) = fs::copy(
            current_dir.join(&self.name),
            git_storage_folder_path.join(&self.name),
        ) {
            return Err(Error::Common(CommonError::Unknown(
                err.to_string(),
                "copy the file",
            )));
        }

        execute_git_commands(&git_storage_folder_path, &self.name)?;

        Ok("Successfully added the file and synchronized the local repository with the remote repository".to_string())
    }
}
