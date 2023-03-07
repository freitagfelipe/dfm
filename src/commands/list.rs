use super::Command;
use crate::utils;
use clap::Args;
use colored::Colorize;
use std::path::Path;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    GetStorageFolderPathError(String),
    #[error("Something wrong happened: {0}, when trying to: {1}")]
    Unknown(String, &'static str),
}

/// Lists all added files
#[derive(Debug, Args)]
pub struct List;

impl Command for List {
    type Error = Error;

    fn execute(self) -> Result<String, Self::Error> {
        let storage_folder_path = match utils::get_storage_folder_path() {
            Ok(path) => path,
            Err(err) => return Err(Error::GetStorageFolderPathError(err.to_string()))
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

        let mut index = 1;

        for entry in WalkDir::new(storage_folder_path).max_depth(1) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => return Err(Error::Unknown(err.to_string(), "get a dir entry")),
            };

            if Path::new(&entry.path()).is_dir() {
                continue;
            }

            let entry = match entry.file_name().to_str() {
                Some(entry) => entry,
                None => {
                    return Err(Error::Unknown(
                        "invalid UTF-8".to_string(),
                        "convert OsStr to str",
                    ))
                }
            };

            println!("{index}. {}", entry.cyan());

            index += 1;
        }

        Ok("Finished listing your files".to_string())
    }
}
