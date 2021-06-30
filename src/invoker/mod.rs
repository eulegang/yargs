use std::borrow::Cow;

mod preview;
mod template;

#[cfg(test)]
mod test;

pub use preview::{DetachedPreview, Preview};
use template::Template;

/// Takes patterns and fills them input and executes command
#[derive(Debug, PartialEq, Eq)]
pub struct Invoker {
    offsets: Vec<usize>,
    templates: Vec<Template>,
}

impl Invoker {
    /// Build an invoker based off of a template
    pub fn new(pattern: &str, command: Vec<String>) -> Invoker {
        let mut offsets = Vec::new();
        let mut templates = Vec::with_capacity(command.len());

        for (i, part) in command.into_iter().enumerate() {
            if part.as_str() == pattern {
                offsets.push(i);
            } else {
                templates.push(Template::new(pattern, part));
            }
        }

        Invoker { offsets, templates }
    }

    /// Creates command invocation preview
    pub fn preview<'s, 'a>(&'s self, inputs: &[&'a str]) -> Preview<'a>
    where
        's: 'a,
    {
        let mut fill =
            Vec::with_capacity(self.offsets.len().max(inputs.len()) + self.templates.len());

        let mut current = 0;

        let mut offset_iter = self.offsets.iter().peekable();
        let mut static_iter = self.templates.iter();
        let mut input_iter = inputs.iter();

        loop {
            if Some(&&current) == offset_iter.peek() {
                if let Some(input) = input_iter.next() {
                    offset_iter.next();
                    fill.push(Cow::from(*input));
                }
            } else {
                if let Some(s) = static_iter.next() {
                    fill.push(Cow::from(s.apply(&mut input_iter)));
                } else if let Some(input) = input_iter.next() {
                    fill.push(Cow::from(*input));
                } else {
                    break;
                }
            }

            current += 1;
        }

        while let Some(input) = input_iter.next() {
            fill.push(Cow::from(*input));
        }

        Preview::new(fill)
    }

    pub fn slots(&self) -> u32 {
        let mut slots = self.offsets.len() as u32;

        for template in &self.templates {
            match template {
                Template::Interp { offsets, .. } => slots += offsets.len() as u32,
                _ => (),
            }
        }

        slots
    }
}
