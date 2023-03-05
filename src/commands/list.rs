use super::Command;
use crate::setup::get_storage_folder_path;
use clap::Args;
use colored::Colorize;
use std::path::Path;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Something wrong happened: {0}")]
    Unknown(String),
}

/// Lists all added files
#[derive(Debug, Args)]
pub struct List;

impl Command for List {
    type Error = Error;

    fn execute(self) -> Result<&'static str, Self::Error> {
        let storage_folder_path = get_storage_folder_path();

        let mut index = 1;

        for entry in WalkDir::new(storage_folder_path).max_depth(1) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => return Err(Error::Unknown(err.to_string())),
            };

            if Path::new(&entry.path()).is_dir() {
                continue;
            }

            let entry = match entry.file_name().to_str() {
                Some(entry) => entry,
                None => {
                    return Err(Error::Unknown(
                        "can not get the file name string".to_string(),
                    ))
                }
            };

            println!("{index}. {}", entry.cyan());

            index += 1;
        }

        Ok("Finished listing your files")
    }
}
