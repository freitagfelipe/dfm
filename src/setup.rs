use crate::error::{CommandError, ExecutionError};
use crate::git::GitCommandExecuterBuilder;
use crate::utils;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("You need to have git installed to use DFM")]
    NeedGit,
}

impl From<Error> for CommandError {
    fn from(err: Error) -> Self {
        CommandError::Usage(err.to_string())
    }
}

fn check_if_git_is_installed() -> Result<(), CommandError> {
    let status = match Command::new("git")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Ok(status) => status,
        Err(err) => {
            return Err(ExecutionError::GitCommand {
                command: "git",
                err: err.to_string(),
            }
            .into());
        }
    };

    let Some(command_code) = status.code() else {
        return Err(ExecutionError::Unknown { err: "process terminated by signal".to_string(), trying_to: "get git status code" }.into());
    };

    if command_code != 1 {
        return Err(Error::NeedGit.into());
    }

    Ok(())
}

pub fn execute_git_commands(git_storage_folder_path: &Path) -> Result<(), ExecutionError> {
    GitCommandExecuterBuilder::new(git_storage_folder_path)
        .run_init()
        .build()
        .run()?;

    Ok(())
}

pub fn setup() -> Result<(), CommandError> {
    check_if_git_is_installed()?;

    let git_storage_folder_path = match utils::get_git_storage_folder_path() {
        Ok(path) => path,
        Err(err) => {
            return Err(ExecutionError::GetStorageFolderPath(err.to_string()).into());
        }
    };

    if git_storage_folder_path.is_dir() {
        return Ok(());
    }

    if let Err(err) = fs::create_dir_all(&git_storage_folder_path) {
        return Err(ExecutionError::CreateStorageFolder(err.to_string()).into());
    }

    let storage_folder_path = match git_storage_folder_path.canonicalize() {
        Ok(path) => path,
        Err(err) => {
            return Err(ExecutionError::CanonicalizePath(err.to_string()).into());
        }
    };

    execute_git_commands(&storage_folder_path)?;

    Ok(())
}
