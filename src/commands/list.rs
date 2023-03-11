use super::Command;
use crate::error::{CommandError, ExecutionError};
use crate::utils;
use clap::Args;
use colored::Colorize;
use std::path::Path;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Your remote repository is empty")]
    EmptyRepository,
}

impl From<Error> for CommandError {
    fn from(err: Error) -> Self {
        CommandError::Usage(err.to_string())
    }
}

/// Lists all the files that are in the remote repository
#[derive(Debug, Args)]
pub struct List;

impl Command for List {
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

        let mut index = 1;

        for entry in WalkDir::new(git_storage_folder_path).max_depth(1) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    return Err(ExecutionError::GetDirEntry(err.to_string()).into());
                }
            };

            if Path::new(&entry.path()).is_dir() {
                continue;
            }

            let entry = match entry.file_name().to_str() {
                Some(entry) => entry,
                None => {
                    return Err(ExecutionError::InvalidUTF8("convert OsStr to &str").into());
                }
            };

            println!("{index}. {}", entry.cyan());

            index += 1;
        }

        if index == 1 {
            return Err(Error::EmptyRepository.into());
        }

        Ok("Finished listing your files".to_string())
    }
}
