use std::{ffi::OsString, io};

use clap::Command;

pub trait Dispatch {
    fn dispatch(self, arguments: Vec<OsString>) -> io::Result<()>;
}

impl<F> Dispatch for F
where
    F: FnOnce(Vec<OsString>) -> io::Result<()>,
{
    fn dispatch(self, arguments: Vec<OsString>) -> io::Result<()> {
        self(arguments)
    }
}

pub fn command() -> Command {
    Command::new("rusty-kode")
        .about("Analyze Python code metrics")
        .version(env!("CARGO_PKG_VERSION"))
}

pub fn run<I, D, W>(arguments: I, dispatch: D, output: &mut W) -> io::Result<()>
where
    I: IntoIterator<Item = OsString>,
    D: Dispatch,
    W: io::Write,
{
    let arguments: Vec<_> = arguments.into_iter().collect();

    if arguments.len() == 1 && arguments[0].to_str() == Some("--version") {
        write!(output, "{}", command().render_version())?;
        return Ok(());
    }

    if arguments.is_empty() {
        command().write_help(output)?;
        writeln!(output)?;
        return Ok(());
    }

    dispatch.dispatch(arguments)
}
