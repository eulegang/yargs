use structopt::StructOpt;

mod cli;

fn main() {
    let cli = cli::Cli::from_args();
    cli.validate();

    dbg!(cli);
}
