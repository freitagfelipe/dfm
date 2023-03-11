use crate::error::{CommandError, ExecutionError};
use std::env;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("You need to set a remote repository before use DFM")]
    SetRemoteRepository,
}

impl From<Error> for CommandError {
    fn from(err: Error) -> Self {
        CommandError::Usage(err.to_string())
    }
}

pub fn get_storage_folder_path() -> Result<PathBuf, ExecutionError> {
    if cfg!(any(target_os = "linux", target_os = "macos")) {
        let home_path = match env::var("HOME") {
            Ok(env) => env,
            Err(err) => {
                return Err(ExecutionError::GetEnvVar {
                    name: "HOME",
                    err: err.to_string(),
                })
            }
        };

        Ok(PathBuf::from(format!("{home_path}/.config/DFM")))
    } else {
        let home_path = match env::var("APPDATA") {
            Ok(env) => env,
            Err(err) => {
                return Err(ExecutionError::GetEnvVar {
                    name: "APPDATA",
                    err: err.to_string(),
                })
            }
        };

        Ok(PathBuf::from(format!("{home_path}\\DFM")))
    }
}

pub fn get_git_storage_folder_path() -> Result<PathBuf, ExecutionError> {
    Ok(get_storage_folder_path()?.join("dotfiles"))
}

pub fn check_if_file_exists(folder: &Path, file_name: &str) -> bool {
    folder.join(file_name).exists()
}

pub fn check_if_remote_link_is_added() -> Result<(), CommandError> {
    let storage_folder_path = get_storage_folder_path()?;

    if !storage_folder_path.join("remote.txt").exists() {
        return Err(Error::SetRemoteRepository.into());
    }

    Ok(())
}
