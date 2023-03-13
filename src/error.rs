use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("Error while trying to get the storage folder: {0}")]
    GetStorageFolderPath(String),
    #[error("Error while trying to canonicalize a path: {0}")]
    CanonicalizePath(String),
    #[error("Error while trying to get the current dir: {0}")]
    GetCurrentDir(String),
    #[error("Error while trying to execute git {command:?}: {err:?}")]
    GitCommand { command: &'static str, err: String },
    #[error("Error while trying to copy a file: {0}")]
    CopyFile(String),
    #[error("Error while trying to create a file: {0}")]
    CreateFile(String),
    #[error("Error while trying to write to a file: {0}")]
    WriteToFile(String),
    #[error("Error while trying to open a file: {0}")]
    OpenFile(String),
    #[error("Error while trying to read a file: {0}")]
    ReadFile(String),
    #[error("Error while trying to remove a file: {0}")]
    RemoveFile(String),
    #[error("Error while trying to get the {name:?} env var: {err:?}")]
    GetEnvVar { name: &'static str, err: String },
    #[error("Error while trying to get a dir entry: {0}")]
    GetDirEntry(String),
    #[error("Error while trying to remove the storage folder: {0}")]
    RemoveStorageFolder(String),
    #[error("Error while trying create the storage folder: {0}")]
    CreateStorageFolder(String),
    #[error("Invalid UTF-8 while trying to: {0}")]
    InvalidUTF8(&'static str),
    #[error("Error while trying to create the ssh regex: {0}")]
    Regex(String),
    #[error("Error while trying to execute {0} non zero status code")]
    NoSuccess(&'static str),
    #[error("Repository not found")]
    RepositoryNotFound,
    #[error("Something wrong happened: {err:?}, while trying to: {trying_to:?}")]
    Unknown {
        err: String,
        trying_to: &'static str,
    },
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("{0}")]
    Usage(String),
    #[error("{0}")]
    Execution(ExecutionError),
}

impl From<ExecutionError> for CommandError {
    fn from(err: ExecutionError) -> Self {
        CommandError::Execution(err)
    }
}
