use super::*;

pub struct Run;

impl Process for Run {
    fn process(&self, preview: &Preview) {
        preview.run().unwrap();
    }

    fn finalize(self) {}
}
