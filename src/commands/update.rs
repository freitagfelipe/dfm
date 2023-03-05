use super::Command;
use crate::utils::{check_if_file_exists, get_storage_folder_path};
use clap::Args;
use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;
use std::process::{Command as Cmd, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File does not exists in the current folder")]
    FileDoesNotExists,
    #[error("File not added")]
    FileNotAdded,
    #[error("Nothing to update")]
    NothingToUpdate,
    #[error("Something wrong happened: {0}")]
    Unknown(String),
}

/// Updates a file from the repository
#[derive(Debug, Args)]
pub struct Update {
    /// File name
    name: String,
}

fn execute_git_commands(storage_folder: &Path, file_name: &str) -> Result<(), Error> {
    if let Err(err) = Cmd::new("git")
        .args(["add", "."])
        .current_dir(storage_folder)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        return Err(Error::Unknown(err.to_string()));
    }

    if let Err(err) = Cmd::new("git")
        .args(["commit", "-m", &format!("\"Update {file_name}\"")])
        .current_dir(storage_folder)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        return Err(Error::Unknown(err.to_string()));
    };

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

    fn execute(self) -> Result<&'static str, Self::Error> {
        let storage_folder_path = match get_storage_folder_path().canonicalize() {
            Ok(path) => path,
            Err(err) => return Err(Error::Unknown(err.to_string())),
        };

        let current_dir = match env::current_dir() {
            Ok(path) => path,
            Err(err) => {
                return Err(Error::Unknown(err.to_string()));
            }
        };

        if !check_if_file_exists(&current_dir, &self.name) {
            return Err(Error::FileDoesNotExists);
        }

        if !check_if_file_exists(&storage_folder_path, &self.name) {
            return Err(Error::FileNotAdded);
        }

        let two_files_are_equal = match check_if_files_are_equal(
            &current_dir.join(&self.name),
            &storage_folder_path.join(&self.name),
        ) {
            Ok(result) => result,
            Err(err) => {
                return Err(Error::Unknown(err.to_string()));
            }
        };

        if two_files_are_equal {
            return Err(Error::NothingToUpdate);
        }

        if fs::copy(
            current_dir.join(&self.name),
            storage_folder_path.join(&self.name),
        )
        .is_err()
        {
            return Err(Error::Unknown(
                "An error ocurred while trying to copy the file".to_string(),
            ));
        }

        execute_git_commands(&storage_folder_path, &self.name)?;

        Ok("Successfully updated the file")
    }
}
