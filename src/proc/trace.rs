use super::*;

pub enum Log<Next: Process> {
    Trace(Next),
    Nop(Next),
}

impl<Next> Process for Log<Next>
where
    Next: Process,
{
    fn process(&self, preview: &Preview) {
        match self {
            Log::Trace(n) => {
                eprintln!("{}", preview);
                n.process(preview);
            }

            Log::Nop(n) => {
                n.process(preview);
            }
        }
    }

    fn finalize(self) {
        match self {
            Log::Trace(n) => n.finalize(),
            Log::Nop(n) => n.finalize(),
        }
    }
}
