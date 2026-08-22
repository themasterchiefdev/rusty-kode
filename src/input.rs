use std::{
    ffi::{OsStr, OsString},
    fs::File,
    io,
    io::{BufRead, BufReader, Read},
    path::Path,
};

pub fn is_python_filename(filename: &OsStr) -> bool {
    filename.as_encoded_bytes().ends_with(b".py")
}

pub fn has_python_shebang(path: &Path) -> bool {
    let Ok(file) = File::open(path) else {
        return false;
    };
    let mut first_line = String::new();
    if BufReader::new(file).read_line(&mut first_line).is_err() {
        return false;
    }

    first_line.starts_with("#!") && first_line.contains("python")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MetricInputOrigin {
    StandardInput,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetricInput {
    display_name: String,
    source: String,
    origin: MetricInputOrigin,
}

impl MetricInput {
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn origin(&self) -> MetricInputOrigin {
        self.origin
    }
}

pub trait MetricInputConsumer {
    fn consume(&mut self, input: MetricInput);
}

pub trait NonStandardInputDelegate {
    fn discover(&mut self, paths: &[OsString]);
}

pub fn discover_inputs<R, C, D>(
    paths: &[OsString],
    standard_input: &mut R,
    consumer: &mut C,
    non_standard_input: &mut D,
) -> io::Result<()>
where
    R: Read,
    C: MetricInputConsumer,
    D: NonStandardInputDelegate,
{
    if paths.is_empty() || !paths.iter().all(|path| path == "-") {
        non_standard_input.discover(paths);
        return Ok(());
    }

    let mut source = String::new();
    standard_input.read_to_string(&mut source)?;
    consumer.consume(MetricInput {
        display_name: "-".to_owned(),
        source,
        origin: MetricInputOrigin::StandardInput,
    });

    Ok(())
}
