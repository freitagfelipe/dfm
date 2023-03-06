use dfm::{cli, setup};

fn main() {
    setup::setup();

    let cli = cli::parse();

    cli.command.invoke();
}
