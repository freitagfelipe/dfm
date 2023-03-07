use std::path::Path;
use std::process::{Child, Command, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Something wrong happened: {0}, when trying to: {1}")]
    Unknown(String, &'static str),
}

pub struct Executer<'a> {
    storage_folder_path: &'a Path,
    commit_message: String,
    remote_link: String,
    run_init: bool,
    run_remote_add: bool,
    run_pull: bool,
    run_commit: bool,
}

impl Executer<'_> {
    pub fn run(self) -> Result<(), Error> {
        if self.run_init {
            if let Err(err) = init(self.storage_folder_path)?.wait() {
                return Err(Error::Unknown(err.to_string(), "wait git init"));
            }
        }

        if self.run_remote_add {
            if let Err(err) = remote_add(self.storage_folder_path, &self.remote_link)?.wait() {
                return Err(Error::Unknown(err.to_string(), "wait git remote add"));
            }
        }

        if self.run_pull || self.run_commit {
            if let Err(err) = pull(self.storage_folder_path)?.wait() {
                return Err(Error::Unknown(err.to_string(), "wait git pull"));
            }
        }

        if !self.run_commit {
            return Ok(());
        }

        if let Err(err) = add_all(self.storage_folder_path)?.wait() {
            return Err(Error::Unknown(err.to_string(), "wait git add"));
        }

        if let Err(err) = commit(self.storage_folder_path, &self.commit_message)?.wait() {
            return Err(Error::Unknown(err.to_string(), "wait git commit"));
        }

        if let Err(err) = push(self.storage_folder_path)?.wait() {
            return Err(Error::Unknown(err.to_string(), "wait git push"));
        }

        Ok(())
    }
}

pub struct ExecuterBuilder<'a> {
    storage_folder_path: &'a Path,
    commit_message: String,
    remote_link: String,
    run_init: bool,
    run_remote_add: bool,
    run_pull: bool,
    run_commit: bool,
}

impl<'a> ExecuterBuilder<'a> {
    pub fn new(storage_folder_path: &'a Path) -> Self {
        ExecuterBuilder {
            storage_folder_path,
            run_init: false,
            commit_message: String::new(),
            remote_link: String::new(),
            run_commit: false,
            run_remote_add: false,
            run_pull: false,
        }
    }

    pub fn run_init(mut self) -> Self {
        self.run_init = true;

        self
    }

    pub fn run_commit(mut self, commit_message: impl Into<String>) -> Self {
        self.run_commit = true;
        self.commit_message = commit_message.into();

        self
    }

    pub fn run_pull(mut self) -> Self {
        self.run_pull = true;

        self
    }

    pub fn run_remote_add(mut self, link: impl Into<String>) -> Self {
        self.run_remote_add = true;
        self.remote_link = link.into();

        self
    }

    pub fn build(self) -> Executer<'a> {
        Executer {
            storage_folder_path: self.storage_folder_path,
            run_init: self.run_init,
            commit_message: self.commit_message,
            remote_link: self.remote_link,
            run_commit: self.run_commit,
            run_remote_add: self.run_remote_add,
            run_pull: self.run_pull,
        }
    }
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
