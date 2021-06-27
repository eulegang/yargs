use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::process::{Command, Stdio};

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, FromRawHandle};

pub struct Preview<'a> {
    args: Vec<Cow<'a, str>>,
}

impl<'a> Preview<'a> {
    pub fn new(args: Vec<Cow<'a, str>>) -> Preview<'a> {
        Preview { args }
    }

    pub fn run(&self) -> io::Result<u8> {
        let status = Command::new(&*self.args[0])
            .args(
                &self.args[1..]
                    .iter()
                    .map(|s| s.as_ref())
                    .collect::<Vec<&str>>(),
            )
            .stdin(Stdio::null())
            .status()?;

        Ok(status.code().unwrap_or(15) as u8)
    }

    pub fn run_interactive(&self, tty: &mut File) -> io::Result<u8> {
        let status = Command::new(&*self.args[0])
            .args(
                &self.args[1..]
                    .iter()
                    .map(|s| s.as_ref())
                    .collect::<Vec<&str>>(),
            )
            .stdin(to_stdio(tty))
            .status()?;

        Ok(status.code().unwrap_or(15) as u8)
    }

    #[cfg(test)]
    pub fn as_strs(&self) -> Vec<&str> {
        let mut vec = Vec::with_capacity(self.args.capacity());

        for arg in &self.args {
            vec.push(arg.as_ref());
        }

        vec
    }
}

impl std::fmt::Display for Preview<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", &self.args[0])?;

        for s in &self.args[1..] {
            write!(fmt, " {}", s)?;
        }

        Ok(())
    }
}

#[cfg(unix)]
fn to_stdio(f: &File) -> Stdio {
    unsafe { Stdio::from_raw_fd(f.as_raw_fd()) }
}

#[cfg(windows)]
fn to_stdio(f: &File) -> Stdio {
    unsafe { Stdio::from_raw_handle(f.as_raw_handle()) }
}
