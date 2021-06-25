use std::fs::File;
use std::io;
use std::process::{Command, Stdio};

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd};

#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, FromRawHandle};

/// Takes patterns and fills them input and executes command
#[derive(Debug, PartialEq, Eq)]
pub struct Invoker {
    offsets: Vec<usize>,
    statics: Vec<String>,
}

impl Invoker {
    /// Build an invoker based off of a template
    pub fn new(pattern: &str, command: Vec<String>) -> Invoker {
        let mut offsets = Vec::new();
        let mut statics = Vec::with_capacity(command.len());

        for (i, part) in command.into_iter().enumerate() {
            if part.as_str() == pattern {
                offsets.push(i);
            } else {
                statics.push(part);
            }
        }

        Invoker { offsets, statics }
    }

    fn run(&self, inputs: &[&str], tty: Option<&File>) -> io::Result<u8> {
        let fills = self.fill(inputs);

        #[cfg(unix)]
        fn to_stdio(f: &File) -> Stdio {
            unsafe { Stdio::from_raw_fd(f.as_raw_fd()) }
        }

        #[cfg(windows)]
        fn to_stdio(f: &File) -> Stdio {
            unsafe { Stdio::from_raw_handle(f.as_raw_handle()) }
        }

        let status = Command::new(&fills[0])
            .args(&fills[1..])
            .stdin(tty.map(to_stdio).unwrap_or(Stdio::null()))
            .status()?;

        Ok(status.code().unwrap_or(15) as u8)
    }

    fn fill<'s, 'a>(&'s self, inputs: &[&'a str]) -> Vec<&'a str>
    where
        's: 'a,
    {
        let mut fill =
            Vec::with_capacity(self.offsets.len().max(inputs.len()) + self.statics.len());

        let mut current = 0;

        let mut offset_iter = self.offsets.iter().peekable();
        let mut static_iter = self.statics.iter();
        let mut input_iter = inputs.iter();

        loop {
            if Some(&&current) == offset_iter.peek() {
                if let Some(input) = input_iter.next() {
                    offset_iter.next();
                    fill.push(*input);
                }
            } else {
                if let Some(s) = static_iter.next() {
                    fill.push(s.as_str());
                } else if let Some(input) = input_iter.next() {
                    fill.push(*input);
                } else {
                    break;
                }
            }

            current += 1;
        }

        while let Some(input) = input_iter.next() {
            fill.push(*input);
        }

        fill
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(
            Invoker::new("%", vec!["echo".to_string(), "%".to_string()]),
            Invoker {
                statics: vec!["echo".to_string()],
                offsets: vec![1]
            },
            "basic 'echo %' case"
        );

        assert_eq!(
            Invoker::new("{}", vec!["echo".to_string(), "%".to_string()]),
            Invoker {
                statics: vec!["echo".to_string(), "%".to_string()],
                offsets: vec![]
            },
            "'echo %' with '{{}}' pattern"
        );

        assert_eq!(
            Invoker::new("{}", vec!["echo".to_string(), "{}".to_string()]),
            Invoker {
                statics: vec!["echo".to_string()],
                offsets: vec![1]
            },
            "'echo {{}}' with '{{}}' pattern"
        );
    }

    #[test]
    fn fill() {
        let echo = Invoker::new("%", vec!["echo".to_string(), "%".to_string()]);

        assert_eq!(
            echo.fill(&["foo"]),
            vec!["echo", "foo"],
            "simple 1 to 1 pattern fill"
        );

        assert_eq!(
            echo.fill(&["foo", "bar"]),
            vec!["echo", "foo", "bar"],
            "simple overfill"
        );

        assert_eq!(echo.fill(&[]), vec!["echo"], "simple underfill");

        let cmd = Invoker::new(
            "%",
            vec![
                "cat".to_string(),
                "%".to_string(),
                "-E".to_string(),
                "%".to_string(),
            ],
        );

        assert_eq!(
            cmd.fill(&["hello", "world"]),
            vec!["cat", "hello", "-E", "world"],
            "multi slot simple fill"
        );

        assert_eq!(
            cmd.fill(&["hello"]),
            vec!["cat", "hello", "-E"],
            "multi slot underfill"
        );

        assert_eq!(
            cmd.fill(&["hello", "world", "foobar"]),
            vec!["cat", "hello", "-E", "world", "foobar"],
            "multi slot overfill"
        );

        assert_eq!(
            cmd.fill(&["hello", "world"]).capacity(),
            4,
            "multi slot simple fill capacity"
        );

        assert_eq!(
            cmd.fill(&["hello", "world", "foobar"]).capacity(),
            5,
            "multi slot overfill capacity"
        );

        // even though 3 elements should exist we want to
        // have a decent heuristic for initial capacity of
        // the result set.
        assert_eq!(
            cmd.fill(&["hello"]).capacity(),
            4,
            "multi slot underfill capacity"
        );
    }
}
