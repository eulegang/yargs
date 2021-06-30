use super::*;

use memchr::memchr;

use std::fs::File;
use std::io::{Read, Write};
use std::str::from_utf8;

pub enum Log<Next: Process> {
    Trace(Next),
    Ask(File, Next),
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

            Log::Ask(tty, n) => loop {
                if let Some(exec) = ask(&tty, preview) {
                    if exec {
                        n.process(preview);
                    }

                    break;
                }
            },
        }
    }

    fn finalize(self) {
        match self {
            Log::Trace(n) => n.finalize(),
            Log::Nop(n) => n.finalize(),
            Log::Ask(_, n) => n.finalize(),
        }
    }
}

fn ask(mut tty: &File, preview: &Preview) -> Option<bool> {
    write!(tty, "exec '{}'? ", preview).unwrap();
    tty.flush().unwrap();

    let mut buf = [0u8; 16];

    tty.read(&mut buf).unwrap();

    let pos = memchr(0, &buf)?;

    let pos = if buf[pos.saturating_sub(1)] == 0xA {
        pos - 1
    } else {
        pos
    };

    let s = from_utf8(&buf[..pos]).ok()?;

    match s.to_ascii_lowercase().as_str() {
        "yes" | "y" | "true" | "t" => Some(true),
        "no" | "n" | "false" | "f" => Some(false),
        _ => None,
    }
}
