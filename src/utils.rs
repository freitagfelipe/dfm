use std::env;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("An error ocurred when trying to get the {0} enviroment variable: {1}")]
    CanNotGetEnviromentVariable(&'static str, String),
}

pub fn get_storage_folder_path() -> Result<PathBuf, Error> {
    if cfg!(any(target_os = "linux", target_os = "macos")) {
        let home_path = match env::var("HOME") {
            Ok(env) => env,
            Err(err) => return Err(Error::CanNotGetEnviromentVariable("HOME", err.to_string())),
        };

        Ok(PathBuf::from(format!("{home_path}/.config/dotfiles")))
    } else {
        let home_path = match env::var("APPDATA") {
            Ok(env) => env,
            Err(err) => {
                return Err(Error::CanNotGetEnviromentVariable(
                    "APPDATA",
                    err.to_string(),
                ))
            }
        };

        Ok(PathBuf::from(format!("{home_path}\\dotfiles")))
    }
}

pub fn check_if_file_exists(folder: &Path, file_name: &str) -> bool {
    folder.join(file_name).exists()
}
