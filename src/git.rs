use crate::error::ExecutionError;
use std::path::Path;
use std::process::{Command, Output, Stdio};

pub struct GitCommandExecuter<'a> {
    git_storage_folder_path: &'a Path,
    commit_message: String,
    remote_link: String,
    run_init: bool,
    run_remote_add: bool,
    run_remote_remove: bool,
    run_pull: bool,
    run_commit: bool,
}

pub struct GitCommandExecuterBuilder<'a> {
    git_storage_folder_path: &'a Path,
    commit_message: String,
    remote_link: String,
    run_init: bool,
    run_remote_add: bool,
    run_remote_remove: bool,
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

impl GitCommandExecuter<'_> {
    pub fn run(self) -> Result<(), ExecutionError> {
        if self.run_init && !init(self.git_storage_folder_path)?.status.success() {
            return Err(ExecutionError::NoSuccess("git init"));
        }

        if self.run_remote_remove {
            if !remote_remove(self.git_storage_folder_path)?
                .status
                .success()
            {
                return Err(ExecutionError::NoSuccess("git remote remove"));
            }

            return Ok(());
        }

        if self.run_remote_add
            && !remote_add(self.git_storage_folder_path, &self.remote_link)?
                .status
                .success()
        {
            return Err(ExecutionError::NoSuccess("git remote add"));
        }

        if (self.run_pull || self.run_commit)
            && !pull(self.git_storage_folder_path)?.status.success()
        {
            return Err(ExecutionError::NoSuccess("git pull"));
        }

        if !self.run_commit {
            return Ok(());
        }

        if !add_all(self.git_storage_folder_path)?.status.success() {
            return Err(ExecutionError::NoSuccess("git add"));
        }

        if !commit(self.git_storage_folder_path, &self.commit_message)?
            .status
            .success()
        {
            return Err(ExecutionError::NoSuccess("git commit"));
        }

        if !push(self.git_storage_folder_path)?.status.success() {
            return Err(ExecutionError::NoSuccess("git push"));
        }

        Ok(())
    }
}

impl<'a> GitCommandExecuterBuilder<'a> {
    pub fn new(git_storage_folder_path: &'a Path) -> Self {
        GitCommandExecuterBuilder {
            git_storage_folder_path,
            run_init: false,
            commit_message: String::new(),
            remote_link: String::new(),
            run_commit: false,
            run_remote_add: false,
            run_pull: false,
            run_remote_remove: false,
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

    pub fn run_remote_remove(mut self) -> Self {
        self.run_remote_remove = true;

        self
    }

    pub fn run_remote_add(mut self, link: impl Into<String>) -> Self {
        self.run_remote_add = true;
        self.remote_link = link.into();

        self
    }

    pub fn build(self) -> GitCommandExecuter<'a> {
        GitCommandExecuter {
            git_storage_folder_path: self.git_storage_folder_path,
            run_init: self.run_init,
            commit_message: self.commit_message,
            remote_link: self.remote_link,
            run_commit: self.run_commit,
            run_remote_add: self.run_remote_add,
            run_pull: self.run_pull,
            run_remote_remove: self.run_remote_remove,
        }
    }
}

fn init(git_storage_folder_path: &Path) -> Result<Output, GitError> {
    let output = match Command::new("git")
        .arg("init")
        .current_dir(git_storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            return Err(GitError {
                command: "init",
                err: err.to_string(),
            });
        }
    };

    Ok(output)
}

fn add_all(git_storage_folder_path: &Path) -> Result<Output, GitError> {
    let output = match Command::new("git")
        .args(["add", "."])
        .current_dir(git_storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            return Err(GitError {
                command: "add",
                err: err.to_string(),
            });
        }
    };

    Ok(output)
}

fn commit(git_storage_folder_path: &Path, commit_name: &str) -> Result<Output, GitError> {
    let output = match Command::new("git")
        .args(["commit", "-m", commit_name])
        .current_dir(git_storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            return Err(GitError {
                command: "commit",
                err: err.to_string(),
            });
        }
    };

    Ok(output)
}

fn push(git_storage_folder_path: &Path) -> Result<Output, GitError> {
    let output = match Command::new("git")
        .args(["push", "origin", "main"])
        .current_dir(git_storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            return Err(GitError {
                command: "push",
                err: err.to_string(),
            });
        }
    };

    Ok(output)
}

fn remote_remove(git_storage_folder_path: &Path) -> Result<Output, GitError> {
    let output = match Command::new("git")
        .args(["remote", "remove", "origin"])
        .current_dir(git_storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            return Err(GitError {
                command: "remote remove",
                err: err.to_string(),
            });
        }
    };

    Ok(output)
}

fn remote_add(git_storage_folder_path: &Path, link: &str) -> Result<Output, GitError> {
    let output = match Command::new("git")
        .args(["remote", "add", "origin", link])
        .current_dir(git_storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            return Err(GitError {
                command: "remote add",
                err: err.to_string(),
            });
        }
    };

    Ok(output)
}

fn pull(git_storage_folder_path: &Path) -> Result<Output, GitError> {
    let output = match Command::new("git")
        .args(["pull", "origin", "main"])
        .current_dir(git_storage_folder_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            return Err(GitError {
                command: "git pull",
                err: err.to_string(),
            });
        }
    };

    Ok(output)
}
