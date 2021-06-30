use super::*;

use std::fs::{File, OpenOptions};
use std::io;

pub struct InterRun {
    tty: File,
}

impl InterRun {
    pub fn new() -> io::Result<InterRun> {
        let tty = OpenOptions::new().write(true).read(true).open("/dev/tty")?;

        Ok(InterRun { tty })
    }
}

impl Process for InterRun {
    fn process(&self, preview: &Preview) {
        preview.run_interactive(&self.tty).unwrap();
    }

    fn finalize(self) {}
}
