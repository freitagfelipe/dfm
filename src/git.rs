use crate::error::ExecutionError;
use std::path::Path;
use std::process::{Child, Command, Stdio};

pub struct Executer<'a> {
    git_storage_folder_path: &'a Path,
    commit_message: String,
    remote_link: String,
    run_init: bool,
    run_remote_add: bool,
    run_pull: bool,
    run_commit: bool,
}

struct GitError {
    command: &'static str,
    err: String,
}

impl From<GitError> for ExecutionError {
    fn from(err: GitError) -> Self {
        ExecutionError::GitCommand {
            command: err.command,
            err: err.err,
        }
    }
}

impl Executer<'_> {
    pub fn run(self) -> Result<(), ExecutionError> {
        if self.run_init {
            if let Err(err) = init(self.git_storage_folder_path)?.wait() {
                return Err(ExecutionError::WaitingCommand {
                    command: "git init",
                    err: err.to_string(),
                });
            }
        }

        if self.run_remote_add {
            if let Err(err) = remote_add(self.git_storage_folder_path, &self.remote_link)?.wait() {
                return Err(ExecutionError::WaitingCommand {
                    command: "git remote add",
                    err: err.to_string(),
                });
            }
        }

        if self.run_pull || self.run_commit {
            if let Err(err) = pull(self.git_storage_folder_path)?.wait() {
                return Err(ExecutionError::WaitingCommand {
                    command: "git pull",
                    err: err.to_string(),
                });
            }
        }

        if !self.run_commit {
            return Ok(());
        }

        if let Err(err) = add_all(self.git_storage_folder_path)?.wait() {
            return Err(ExecutionError::WaitingCommand {
                command: "git add",
                err: err.to_string(),
            });
        }

        if let Err(err) = commit(self.git_storage_folder_path, &self.commit_message)?.wait() {
            return Err(ExecutionError::WaitingCommand {
                command: "git commit",
                err: err.to_string(),
            });
        }

        if let Err(err) = push(self.git_storage_folder_path)?.wait() {
            return Err(ExecutionError::WaitingCommand {
                command: "git push",
                err: err.to_string(),
            });
        }

        Ok(())
    }
}

pub struct ExecuterBuilder<'a> {
    git_storage_folder_path: &'a Path,
    commit_message: String,
    remote_link: String,
    run_init: bool,
    run_remote_add: bool,
    run_pull: bool,
    run_commit: bool,
}

impl<'a> ExecuterBuilder<'a> {
    pub fn new(git_storage_folder_path: &'a Path) -> Self {
        ExecuterBuilder {
            git_storage_folder_path,
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
            git_storage_folder_path: self.git_storage_folder_path,
            run_init: self.run_init,
            commit_message: self.commit_message,
            remote_link: self.remote_link,
            run_commit: self.run_commit,
            run_remote_add: self.run_remote_add,
            run_pull: self.run_pull,
        }
    }
}

fn init(git_storage_folder_path: &Path) -> Result<Child, GitError> {
    let handler = match Command::new("git")
        .arg("init")
        .current_dir(git_storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(handler) => handler,
        Err(err) => {
            return Err(GitError {
                command: "init",
                err: err.to_string(),
            });
        }
    };

    Ok(handler)
}

fn add_all(git_storage_folder_path: &Path) -> Result<Child, GitError> {
    let handler = match Command::new("git")
        .args(["add", "."])
        .current_dir(git_storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(handler) => handler,
        Err(err) => {
            return Err(GitError {
                command: "add",
                err: err.to_string(),
            });
        }
    };

    Ok(handler)
}

fn commit(git_storage_folder_path: &Path, commit_name: &str) -> Result<Child, GitError> {
    let handler = match Command::new("git")
        .args(["commit", "-m", commit_name])
        .current_dir(git_storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(handler) => handler,
        Err(err) => {
            return Err(GitError {
                command: "commit",
                err: err.to_string(),
            });
        }
    };

    Ok(handler)
}

fn push(git_storage_folder_path: &Path) -> Result<Child, GitError> {
    let handler = match Command::new("git")
        .args(["push", "origin", "main"])
        .current_dir(git_storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(handler) => handler,
        Err(err) => {
            return Err(GitError {
                command: "push",
                err: err.to_string(),
            });
        }
    };

    Ok(handler)
}

fn remote_add(git_storage_folder_path: &Path, link: &str) -> Result<Child, GitError> {
    let handler = match Command::new("git")
        .args(["remote", "add", "origin", link])
        .current_dir(git_storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(handler) => handler,
        Err(err) => {
            return Err(GitError {
                command: "remote add",
                err: err.to_string(),
            });
        }
    };

    Ok(handler)
}

fn pull(git_storage_folder_path: &Path) -> Result<Child, GitError> {
    let handler = match Command::new("git")
        .args(["pull", "origin", "main"])
        .current_dir(git_storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(handler) => handler,
        Err(err) => {
            return Err(GitError {
                command: "git pull",
                err: err.to_string(),
            });
        }
    };

    Ok(handler)
}
