use std::{ffi::OsString, io};

use clap::Command;

pub fn command() -> Command {
    Command::new("rusty-kode").about("Analyze Python code metrics")
}

pub fn run<I, D, W>(arguments: I, dispatch: D, output: &mut W) -> io::Result<()>
where
    I: IntoIterator<Item = OsString>,
    D: FnOnce(Vec<OsString>) -> io::Result<()>,
    W: io::Write,
{
    let arguments: Vec<_> = arguments.into_iter().collect();

    if arguments.is_empty() {
        command().write_help(output)?;
        writeln!(output)?;
        return Ok(());
    }

    dispatch(arguments)
}
