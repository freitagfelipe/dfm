use colored::Colorize;
use dfm::{cli, setup};

fn main() {
    if let Err(err) = setup() {
        eprintln!("{}", err.to_string().red());

        return;
    }

    let cli = cli::parse();

    cli.command.invoke();
}
