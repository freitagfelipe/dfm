use crate::git::ExecuterBuilder;
use crate::utils::{self, CommonError};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("You need to have git installed to use DFM")]
    NeedGit,
    #[error("{0}")]
    Common(CommonError),
}

fn check_if_git_is_installed() -> Result<(), Error> {
    let status = match Command::new("git")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Ok(status) => status,
        Err(err) => {
            return Err(Error::Common(CommonError::Unknown(
                err.to_string(),
                "execute git",
            )))
        }
    };

    let Some(command_code) = status.code() else {
        return Err(Error::Common(CommonError::Unknown("process terminated by signal".to_string(), "get git status code")));
    };

    if command_code != 1 {
        return Err(Error::NeedGit);
    }

    Ok(())
}

pub fn execute_git_commands(git_storage_folder_path: &Path) -> Result<(), Error> {
    if let Err(err) = ExecuterBuilder::new(git_storage_folder_path)
        .run_init()
        .build()
        .run()
    {
        return Err(Error::Common(CommonError::GitCommand(err.to_string())));
    }

    Ok(())
}

pub fn setup() -> Result<(), Error> {
    check_if_git_is_installed()?;

    let git_storage_folder_path = match utils::get_git_storage_folder_path() {
        Ok(path) => path,
        Err(err) => {
            return Err(Error::Common(CommonError::GetStorageFolderPath(
                err.to_string(),
            )))
        }
    };

    if git_storage_folder_path.is_dir() {
        return Ok(());
    }

    if let Err(err) = fs::create_dir_all(&git_storage_folder_path) {
        return Err(Error::Common(CommonError::Unknown(
            err.to_string(),
            "create the storage folder",
        )));
    }

    let storage_folder_path = match git_storage_folder_path.canonicalize() {
        Ok(path) => path,
        Err(err) => {
            return Err(Error::Common(CommonError::Unknown(
                err.to_string(),
                "canonicalize the storage folder path",
            )));
        }
    };

    execute_git_commands(&storage_folder_path)?;

    Ok(())
}
