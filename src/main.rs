use structopt::StructOpt;

mod cli;
mod invoker;
mod source;

use cli::Cli;
use invoker::Invoker;
use source::Source;

fn main() {
    let cli = Cli::from_args();
    cli.validate();

    let invoker = Invoker::new(&cli.pattern, cli.command);

    let mut src = match cli.args {
        Some(p) => File::open(p).unwrap().into(),
        None => Source::default(),
    };

    let source = src.buffer();

    let splits = source.split(if cli.null_separated { 0x0 } else { 0xA });

    for split in splits {
        let split = split.unwrap();

        let input = String::from_utf8(split).unwrap();
    }
}
