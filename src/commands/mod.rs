mod add;

pub use add::Add;
use colored::Colorize;

pub trait Command: Sized {
    type Error: std::error::Error;

    fn execute(self) -> Result<&'static str, Self::Error>;

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
