#![allow(dead_code)]

use std::{
    cell::{Cell, RefCell},
    io::{self, Cursor, Read},
    process::{Command, Output},
    rc::Rc,
};

use rusty_kode::MetricInput;

pub const FEATURE_ID: &str = "MET-003";
pub const AZURE_WORK_ITEM: u32 = 243;
pub const RADON_REFERENCE: &str = "54b88e5878b2724bf4d77f97349588b811abdff2";

pub fn run_rusty_kode_without_arguments() -> Output {
    Command::new(env!("CARGO_BIN_EXE_rusty-kode"))
        .output()
        .expect("MET-003 acceptance binary should be executable")
}

pub fn combined_output(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

pub fn evidence_context() -> String {
    format!("feature={FEATURE_ID}, azure_work_item={AZURE_WORK_ITEM}, reference={RADON_REFERENCE}")
}

pub const STANDARD_INPUT_FEATURE_ID: &str = "MET-007";
pub const STANDARD_INPUT_AZURE_WORK_ITEM: u32 = 247;

pub fn standard_input_evidence_context() -> String {
    format!(
        "feature={STANDARD_INPUT_FEATURE_ID}, azure_work_item={STANDARD_INPUT_AZURE_WORK_ITEM}, reference={RADON_REFERENCE}"
    )
}

pub struct CountingReader {
    inner: Cursor<Vec<u8>>,
    accesses: Rc<Cell<usize>>,
}

impl CountingReader {
    pub fn new(source: &str) -> (Self, Rc<Cell<usize>>) {
        let accesses = Rc::new(Cell::new(0));
        (
            Self {
                inner: Cursor::new(source.as_bytes().to_vec()),
                accesses: Rc::clone(&accesses),
            },
            accesses,
        )
    }
}

impl Read for CountingReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.accesses.set(self.accesses.get() + 1);
        self.inner.read(buffer)
    }
}

pub struct FailingReader;

impl Read for FailingReader {
    fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(
            io::ErrorKind::BrokenPipe,
            "injected standard-input failure",
        ))
    }
}

#[derive(Clone, Default)]
pub struct RecordingMetricInputConsumer {
    inputs: Rc<RefCell<Vec<MetricInput>>>,
}

impl RecordingMetricInputConsumer {
    pub fn inputs(&self) -> Rc<RefCell<Vec<MetricInput>>> {
        Rc::clone(&self.inputs)
    }
}

impl rusty_kode::MetricInputConsumer for RecordingMetricInputConsumer {
    fn consume(&mut self, input: MetricInput) {
        self.inputs.borrow_mut().push(input);
    }
}
