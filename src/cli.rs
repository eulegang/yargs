use std::path::PathBuf;
use structopt::StructOpt;

/// run mutliple commands based off of input
#[derive(StructOpt, Debug)]
pub struct Cli {
    /// separate input by the null character
    #[structopt(short = "0", long = "null")]
    pub null_separated: bool,

    /// read file instead of stdin
    #[structopt(short = "a", long = "args")]
    pub args: Option<PathBuf>,

    /// join input lines into an invocation
    #[structopt(short = "j", long = "join")]
    pub join: bool,

    /// join a certain amount of input lines
    #[structopt(short = "l", long = "lines")]
    pub lines: Option<u32>,

    /// run n parallel jobs (defaults to the number of cores)
    #[structopt(short = "p", long = "parallel")]
    pub parallel: Option<u16>,

    /// open /dev/tty before running a command (implies --parallel 1)
    #[structopt(short = "T", long = "tty")]
    pub tty: bool,

    /// Ask permission before running (implies -t)
    #[structopt(short = "A", long = "ask")]
    pub ask: bool,

    /// trace executions
    #[structopt(short = "t", long = "trace")]
    pub trace: bool,

    /// input substitution pattern
    #[structopt(short = "P", long = "pattern", default_value = "%")]
    pub pattern: String,

    /// Command pattern to run (if no pattern is specified, the end is implied)
    pub command: Vec<String>,
}

impl Cli {
    /// validates cli arguments
    ///
    /// prints error messages and exits if validation fails
    pub fn validate(&self) {
        if let Some(msg) = self.validation_message() {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
    }

    pub fn fill_parallel(mut self) -> Cli {
        if self.parallel.is_none() && !self.ask && !self.tty && !self.join {
            self.parallel = Some(num_cpus::get() as u16);
        }

        self
    }

    fn validation_message(&self) -> Option<&'static str> {
        if self.join && self.lines.is_some() {
            return Some(
                "arbitrary join and join on a certain number of lines may not both be specified",
            );
        }

        if self.lines == Some(0) {
            return Some("Lines may not be zero");
        }

        if self.is_parallel() {
            if self.tty {
                return Some("parallel jobs may not be interactive");
            }

            if self.ask {
                return Some("parallel jobs may not ask before execution");
            }

            if self.join {
                return Some("one job may not be parallel");
            }
        }

        if self.command.is_empty() {
            return Some("a command must be specified to execute");
        }

        if &self.command[0] == &self.pattern {
            return Some("a command may not start with the pattern");
        }

        None
    }

    fn is_parallel(&self) -> bool {
        !matches!(self.parallel, Some(1) | None)
    }
}
