use crate::invoker::Preview;
use crate::Cli;
use std::fs::OpenOptions;

mod inter;
mod par;
mod run;

mod trace;

pub use inter::InterRun;
pub use par::ParRun;
pub use run::Run;

pub use trace::Log;

pub trait Process: Sized {
    fn process(&self, preview: &Preview);
    fn finalize(self);
}

pub enum URun {
    Inter(InterRun),
    Base(Run),
    Parallel(ParRun),
}

pub fn process(cli: &Cli) -> Log<URun> {
    let run = match cli.parallel {
        None | Some(1) => {
            if cli.tty {
                URun::Inter(InterRun::new().unwrap())
            } else {
                URun::Base(Run)
            }
        }
        Some(p) => URun::Parallel(ParRun::new(p)),
    };

    if cli.ask {
        let tty = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/tty")
            .unwrap();
        Log::Ask(tty, run)
    } else if cli.trace {
        Log::Trace(run)
    } else {
        Log::Nop(run)
    }
}

impl Process for URun {
    fn process(&self, preview: &Preview) {
        match self {
            URun::Inter(run) => run.process(preview),
            URun::Base(run) => run.process(preview),
            URun::Parallel(run) => run.process(preview),
        }
    }

    fn finalize(self) {
        match self {
            URun::Inter(run) => run.finalize(),
            URun::Base(run) => run.finalize(),
            URun::Parallel(run) => run.finalize(),
        }
    }
}
