use std::{ffi::OsString, io, io::Read};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetricInput {
    display_name: String,
    source: String,
}

impl MetricInput {
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

pub trait MetricInputConsumer {
    fn consume(&mut self, input: MetricInput);
}

pub fn discover_inputs<R, C>(
    paths: &[OsString],
    standard_input: &mut R,
    consumer: &mut C,
) -> io::Result<()>
where
    R: Read,
    C: MetricInputConsumer,
{
    if paths != [OsString::from("-")] {
        return Ok(());
    }

    let mut source = String::new();
    standard_input.read_to_string(&mut source)?;
    consumer.consume(MetricInput {
        display_name: "-".to_owned(),
        source,
    });

    Ok(())
}
