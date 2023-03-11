mod add;
mod clone;
mod list;
mod remote;
mod remove;
mod reset;
mod sync;
mod update;

use crate::{error::CommandError, utils::write_to_log_file};
pub use add::Add;
pub use clone::Clone;
use colored::Colorize;
pub use list::List;
pub use remote::Remote;
pub use remove::Remove;
pub use reset::Reset;
pub use sync::Sync;
pub use update::Update;

pub trait Command: Sized {
    fn execute(self) -> Result<String, CommandError>;

    fn error(err: CommandError) {
        if let CommandError::Usage(err) = err {
            eprintln!("{}", err.red());

            return;
        }

        eprintln!(
            "{}",
            "Something goes wrong during the command executation, please try again".red()
        );

        if let Err(err) = write_to_log_file(&err.to_string()) {
            eprintln!("{}: {}", "Error while trying to write in the log file".red(), err.to_string().red());
        }
    }

    fn call(self) {
        match self.execute() {
            Ok(message) => println!("{}", message.green()),
            Err(err) => Self::error(err),
        }
    }
}
