use structopt::StructOpt;

mod cli;
mod invoker;

use cli::Cli;
use invoker::Invoker;

fn main() {
    let cli = Cli::from_args();
    cli.validate();

    let invoker = Invoker::new(&cli.pattern, cli.command);
}
