use super::Command;
use crate::git::ExecuterBuilder;
use crate::utils::{self, CommonError};
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
    #[error("{0}")]
    Common(CommonError),
}

/// Updates a file from the remote repository
#[derive(Debug, Args)]
pub struct Update {
    /// File name
    name: String,
}

fn execute_git_commands(git_storage_folder_path: &Path, file_name: &str) -> Result<(), Error> {
    if let Err(err) = ExecuterBuilder::new(git_storage_folder_path)
        .run_commit(&format!("Update {file_name}"))
        .build()
        .run()
    {
        return Err(Error::Common(CommonError::GitCommand(err.to_string())));
    }

    Ok(())
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

        if !utils::check_if_file_exists(&git_storage_folder_path, &self.name) {
            return Err(Error::FileNotAdded);
        }

        let two_files_are_equal = match check_if_files_are_equal(
            &current_dir.join(&self.name),
            &git_storage_folder_path.join(&self.name),
        ) {
            Ok(result) => result,
            Err(err) => {
                return Err(Error::Common(CommonError::Unknown(
                    err.to_string(),
                    "check if files are equal",
                )));
            }
        };

        if two_files_are_equal {
            return Err(Error::NothingToUpdate);
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

        Ok("Successfully updated the file and synchronized the local repository with the remote repository".to_string())
    }
}
