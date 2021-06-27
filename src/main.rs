use std::fs::File;
use std::io::BufRead;
use structopt::StructOpt;

mod cli;
mod collector;
mod invoker;
mod source;

use cli::Cli;
use collector::{Collector, Limit};
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

    let limit = match (cli.join, cli.lines) {
        (true, _) => Limit::Unlimited,
        (_, Some(s)) => s.into(),
        (_, None) => 1.into(),
    };

    let mut collector = Collector::new(limit);

    let source = src.buffer();

    let splits = source.split(if cli.null_separated { 0x0 } else { 0xA });

    for split in splits {
        let split = split.unwrap();

        let input = String::from_utf8(split).unwrap();

        collector.push(input);

        if collector.full() {
            invoker.preview(&collector.refs()).run().unwrap();
            collector.clear();
        }
    }

    if !collector.is_empty() {
        invoker.preview(&collector.refs()).run().unwrap();
    }
}
