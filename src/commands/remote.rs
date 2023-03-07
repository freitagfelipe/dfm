use super::Command;
use crate::utils::get_storage_folder_path;
use clap::{Args, Subcommand};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command as Cmd, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Remote repository already added if you want to change you need to reset DFM")]
    AlreadyAdded,
    #[error("Remote repository not setted yet")]
    NotSetted,
    #[error("Something wrong happened: {0}, when trying to: {1}")]
    Unknown(String, &'static str),
}

/// Manages the remote repository.
#[derive(Debug, Args)]
pub struct Remote {
    #[command(subcommand)]
    subcommands: Subcommands,
}

/// Sets the remote repository to the passed link. You can only do that once without resetting DFM.
#[derive(Debug, Args)]
pub struct Set {
    /// Link to the remote repository
    link: String,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    /// Shows the current remote repository
    Show,
    /// Sets (if not setted yet) the remote repository
    Set(Set),
}

fn execute_git_command(storage_folder_path: &Path, link: &str) -> Result<(), Error> {
    let mut handler = match Cmd::new("git")
        .args(["remote", "add", "origin", link])
        .current_dir(storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(handler) => handler,
        Err(err) => {
            return Err(Error::Unknown(
                err.to_string(),
                "execute git remote add origin",
            ))
        }
    };

    if let Err(err) = handler.wait() {
        return Err(Error::Unknown(
            err.to_string(),
            "wait git remote add origin",
        ));
    }

    let mut handler = match Cmd::new("git")
        .args(["pull", "origin", "main"])
        .current_dir(storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(handler) => handler,
        Err(err) => {
            return Err(Error::Unknown(
                err.to_string(),
                "execute git pull origin main",
            ))
        }
    };

    if let Err(err) = handler.wait() {
        return Err(Error::Unknown(err.to_string(), "wait git pull origin main"));
    }

    Ok(())
}

fn set_remote_link(storage_folder_path: &Path, link: &str) -> Result<String, Error> {
    if get_storage_folder_path().join("remote.txt").exists() {
        return Err(Error::AlreadyAdded);
    }

    let mut file = match File::create(storage_folder_path.join("remote.txt")) {
        Ok(file) => file,
        Err(err) => return Err(Error::Unknown(err.to_string(), "create a file")),
    };

    if let Err(err) = file.write_all(link.as_bytes()) {
        return Err(Error::Unknown(err.to_string(), "write to a file"));
    }

    execute_git_command(storage_folder_path, link)?;

    Ok("Successfully setted the origin".to_string())
}

fn show_remote_link(storage_folder_path: &Path) -> Result<String, Error> {
    if !storage_folder_path.join("remote.txt").exists() {
        return Err(Error::NotSetted);
    }

    let mut file = match File::open(storage_folder_path.join("remote.txt")) {
        Ok(file) => file,
        Err(err) => return Err(Error::Unknown(err.to_string(), "open a file")),
    };

    let mut content = Vec::new();

    if let Err(err) = file.read_to_end(&mut content) {
        return Err(Error::Unknown(err.to_string(), "read a file"));
    }

    let Ok(content) = String::from_utf8(content) else {
        return Err(Error::Unknown("Invalid UTF-8".to_string(), "convert vec<u8> to String"));
    };

    Ok(content)
}

impl Command for Remote {
    type Error = Error;

    fn execute(self) -> Result<String, Self::Error> {
        let storage_folder_path = match get_storage_folder_path().canonicalize() {
            Ok(path) => path,
            Err(err) => {
                return Err(Error::Unknown(
                    err.to_string(),
                    "canonicalize the storage folder path",
                ))
            }
        };

        match self.subcommands {
            Subcommands::Show => show_remote_link(&storage_folder_path),
            Subcommands::Set(set) => set_remote_link(&storage_folder_path, &set.link),
        }
    }
}
