use std::path::Path;
use std::process::{Child, Command, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Something wrong happened: {0}, when trying to: {1}")]
    Unknown(String, &'static str),
}

pub fn init(storage_folder_path: &Path) -> Result<Child, Error> {
    let handler = match Command::new("git")
        .arg("init")
        .current_dir(storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(handler) => handler,
        Err(err) => {
            return Err(Error::Unknown(err.to_string(), "execute git init"));
        }
    };

    Ok(handler)
}

pub fn add_all(storage_folder_path: &Path) -> Result<Child, Error> {
    let handler = match Command::new("git")
        .args(["add", "."])
        .current_dir(storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(handler) => handler,
        Err(err) => return Err(Error::Unknown(err.to_string(), "execute git add .")),
    };

    Ok(handler)
}

pub fn commit(storage_folder_path: &Path, commit_name: &str) -> Result<Child, Error> {
    let handler = match Command::new("git")
        .args(["commit", "-m", commit_name])
        .current_dir(storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(handler) => handler,
        Err(err) => return Err(Error::Unknown(err.to_string(), "execute git commit")),
    };

    Ok(handler)
}

pub fn push(storage_folder_path: &Path) -> Result<Child, Error> {
    let handler = match Command::new("git")
        .args(["push", "origin", "main"])
        .current_dir(storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(handler) => handler,
        Err(err) => {
            return Err(Error::Unknown(
                err.to_string(),
                "execute git push origin main",
            ))
        }
    };

    Ok(handler)
}

pub fn remote_add(storage_folder_path: &Path, link: &str) -> Result<Child, Error> {
    let handler = match Command::new("git")
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

    Ok(handler)
}

pub fn pull(storage_folder_path: &Path) -> Result<Child, Error> {
    let handler = match Command::new("git")
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

    Ok(handler)
}
