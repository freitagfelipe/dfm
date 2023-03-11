mod add;
mod clone;
mod list;
mod remote;
mod remove;
mod reset;
mod update;

use crate::error::CommandError;
pub use add::Add;
pub use clone::Clone;
use colored::Colorize;
pub use list::List;
pub use remote::Remote;
pub use remove::Remove;
pub use reset::Reset;
pub use update::Update;

pub trait Command: Sized {
    fn execute(self) -> Result<String, CommandError>;

    fn error(err: CommandError) {
        println!("{}", err.to_string().red());
    }

    fn call(self) {
        match self.execute() {
            Ok(message) => println!("{}", message.green()),
            Err(err) => Self::error(err),
        }
    }
}
