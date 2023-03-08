use std::env;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommonError {
    #[error("{0}")]
    GetStorageFolderPath(String),
    #[error("{0}")]
    GitCommand(String),
    #[error("You need to set a remote repository before use DFM")]
    SetRemoteRepository,
    #[error("Something wrong happened: {0}, when trying to: {1}")]
    Unknown(String, &'static str),
}

pub fn get_storage_folder_path() -> Result<PathBuf, CommonError> {
    if cfg!(any(target_os = "linux", target_os = "macos")) {
        let home_path = match env::var("HOME") {
            Ok(env) => env,
            Err(err) => return Err(CommonError::Unknown(err.to_string(), "get HOME env var")),
        };

        Ok(PathBuf::from(format!("{home_path}/.config/DFM")))
    } else {
        let home_path = match env::var("APPDATA") {
            Ok(env) => env,
            Err(err) => return Err(CommonError::Unknown(err.to_string(), "get APPDATA env var")),
        };

        Ok(PathBuf::from(format!("{home_path}\\DFM")))
    }
}

pub fn get_git_storage_folder_path() -> Result<PathBuf, CommonError> {
    Ok(get_storage_folder_path()?.join("dotfiles"))
}

pub fn check_if_file_exists(folder: &Path, file_name: &str) -> bool {
    folder.join(file_name).exists()
}

pub fn check_if_remote_link_is_added() -> Result<(), CommonError> {
    let storage_folder_path = get_storage_folder_path()?;

    if !storage_folder_path.join("remote.txt").exists() {
        return Err(CommonError::SetRemoteRepository);
    }

    Ok(())
}
