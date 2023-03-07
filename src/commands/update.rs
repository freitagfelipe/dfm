use super::Command;
use crate::{git, utils};
use clap::Args;
use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File does not exists in the current folder")]
    FileDoesNotExists,
    #[error("File not added")]
    FileNotAdded,
    #[error("Nothing to update")]
    NothingToUpdate,
    #[error("{0}")]
    GetStorageFolderPath(String),
    #[error("{0}")]
    GitCommand(String),
    #[error("Something wrong happened: {0}, when trying to: {1}")]
    Unknown(String, &'static str),
}

/// Updates a file from the repository
#[derive(Debug, Args)]
pub struct Update {
    /// File name
    name: String,
}

fn execute_git_commands(storage_folder_path: &Path, file_name: &str) -> Result<(), Error> {
    let mut handler = match git::add_all(storage_folder_path) {
        Ok(handler) => handler,
        Err(err) => return Err(Error::GitCommand(err.to_string())),
    };

    if let Err(err) = handler.wait() {
        return Err(Error::Unknown(err.to_string(), "wait git add finish"));
    }

    let mut handler = match git::commit(storage_folder_path, &format!("Update {file_name}")) {
        Ok(handler) => handler,
        Err(err) => return Err(Error::GitCommand(err.to_string())),
    };

    if let Err(err) = handler.wait() {
        return Err(Error::Unknown(err.to_string(), "wait git commit finish"));
    }

    let mut handler = match git::push(storage_folder_path) {
        Ok(handler) => handler,
        Err(err) => return Err(Error::GitCommand(err.to_string())),
    };

    if let Err(err) = handler.wait() {
        return Err(Error::Unknown(err.to_string(), "wait git push finish"));
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
        let storage_folder_path = match utils::get_storage_folder_path() {
            Ok(path) => path,
            Err(err) => return Err(Error::GetStorageFolderPath(err.to_string())),
        };

        let storage_folder_path = match storage_folder_path.canonicalize() {
            Ok(path) => path,
            Err(err) => {
                return Err(Error::Unknown(
                    err.to_string(),
                    "canonicalize the storage folder path",
                ))
            }
        };

        let current_dir = match env::current_dir() {
            Ok(path) => path,
            Err(err) => {
                return Err(Error::Unknown(err.to_string(), "get the current dir"));
            }
        };

        if !utils::check_if_file_exists(&current_dir, &self.name) {
            return Err(Error::FileDoesNotExists);
        }

        if !utils::check_if_file_exists(&storage_folder_path, &self.name) {
            return Err(Error::FileNotAdded);
        }

        let two_files_are_equal = match check_if_files_are_equal(
            &current_dir.join(&self.name),
            &storage_folder_path.join(&self.name),
        ) {
            Ok(result) => result,
            Err(err) => {
                return Err(Error::Unknown(err.to_string(), "check if files are equal"));
            }
        };

        if two_files_are_equal {
            return Err(Error::NothingToUpdate);
        }

        if let Err(err) = fs::copy(
            current_dir.join(&self.name),
            storage_folder_path.join(&self.name),
        ) {
            return Err(Error::Unknown(err.to_string(), "copy the file"));
        }

        execute_git_commands(&storage_folder_path, &self.name)?;

        Ok("Successfully updated the file".to_string())
    }
}
