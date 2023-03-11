use super::Command;
use crate::error::{CommandError, ExecutionError};
use crate::git::GitCommandExecuterBuilder;
use crate::utils;
use clap::Args;
use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File does not exist in the current folder")]
    FileDoesNotExists,
    #[error("File not added")]
    FileNotAdded,
    #[error("Nothing to update")]
    NothingToUpdate,
    #[error("You need to set a remote repository before use DFM")]
    SetRemoteRepository,
}

impl From<Error> for CommandError {
    fn from(err: Error) -> Self {
        CommandError::Usage(err.to_string())
    }
}

/// Updates a file from the remote repository
#[derive(Debug, Args)]
pub struct Update {
    /// File name
    name: String,
}

fn check_if_files_are_equal(first_file: &Path, second_file: &Path) -> Result<bool, std::io::Error> {
    let first_file = File::open(first_file)?;
    let second_file = File::open(second_file)?;

    let first_file_metadata = first_file.metadata()?;
    let second_file_metadata = second_file.metadata()?;

    if first_file_metadata.len() != second_file_metadata.len() {
        return Ok(false);
    }

    let first_file_reader = BufReader::new(first_file);
    let second_file_reader = BufReader::new(second_file);

    for (b1, b2) in first_file_reader.bytes().zip(second_file_reader.bytes()) {
        if b1? != b2? {
            return Ok(false);
        }
    }

    Ok(true)
}

impl Command for Update {
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

        if !utils::check_if_file_exists(&git_storage_folder_path, &self.name) {
            return Err(Error::FileNotAdded.into());
        }

        let two_files_are_equal = match check_if_files_are_equal(
            &current_dir.join(&self.name),
            &git_storage_folder_path.join(&self.name),
        ) {
            Ok(result) => result,
            Err(err) => {
                return Err(ExecutionError::Unknown {
                    err: err.to_string(),
                    trying_to: "check if files are equal",
                }
                .into());
            }
        };

        if two_files_are_equal {
            return Err(Error::NothingToUpdate.into());
        }

        if let Err(err) = fs::copy(
            current_dir.join(&self.name),
            git_storage_folder_path.join(&self.name),
        ) {
            return Err(ExecutionError::CopyFile(err.to_string()).into());
        }

        GitCommandExecuterBuilder::new(&git_storage_folder_path)
            .run_commit(&format!("Update {}", self.name))
            .build()
            .run()?;

        Ok("Successfully updated the file and synchronized the local repository with the remote repository".to_string())
    }
}
