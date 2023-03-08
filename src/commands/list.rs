use super::Command;
use crate::git::ExecuterBuilder;
use crate::utils::{self, CommonError};
use clap::Args;
use colored::Colorize;
use std::path::Path;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Your remote repository is empty")]
    EmptyRepository,
    #[error("{0}")]
    Common(CommonError),
}

/// Lists all the files that are in the remote repository
#[derive(Debug, Args)]
pub struct List;

impl Command for List {
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

        if let Err(err) = ExecuterBuilder::new(&git_storage_folder_path)
            .run_pull()
            .build()
            .run()
        {
            return Err(Error::Common(CommonError::GitCommand(err.to_string())));
        }

        if utils::check_if_remote_link_is_added().is_err() {
            return Err(Error::Common(CommonError::SetRemoteRepository));
        }

        let mut index = 1;

        for entry in WalkDir::new(git_storage_folder_path).max_depth(1) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    return Err(Error::Common(CommonError::Unknown(
                        err.to_string(),
                        "get a dir entry",
                    )))
                }
            };

            if Path::new(&entry.path()).is_dir() {
                continue;
            }

            let entry = match entry.file_name().to_str() {
                Some(entry) => entry,
                None => {
                    return Err(Error::Common(CommonError::Unknown(
                        "invalid UTF-8".to_string(),
                        "convert OsStr to str",
                    )))
                }
            };

            println!("{index}. {}", entry.cyan());

            index += 1;
        }

        if index == 1 {
            return Err(Error::EmptyRepository);
        }

        Ok("Finished listing your files".to_string())
    }
}
