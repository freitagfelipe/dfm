mod add;
mod list;
mod remote;
mod remove;
mod reset;
mod update;

pub use add::Add;
use colored::Colorize;
pub use list::List;
pub use remote::Remote;
pub use remove::Remove;
pub use reset::Reset;
use std::error;
pub use update::Update;

pub trait Command: Sized {
    type Error: error::Error;

    fn execute(self) -> Result<String, Self::Error>;

    fn error(err: Self::Error) {
        println!("{}", err.to_string().red());
    }

    fn call(self) {
        match self.execute() {
            Ok(message) => println!("{}", message.green()),
            Err(err) => Self::error(err),
        }
    }
}
