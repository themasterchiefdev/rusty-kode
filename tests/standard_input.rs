mod support;

use std::{ffi::OsString, io};

use support::{
    CountingReader, FailingReader, RecordingMetricInputConsumer, standard_input_evidence_context,
};

#[test]
fn single_standard_input_is_handed_off_atomically() {
    let evidence = standard_input_evidence_context();
    let source = "def answer():\n    return 42\n";
    let paths = [OsString::from("-")];
    let (mut reader, accesses) = CountingReader::new(source);
    let mut consumer = RecordingMetricInputConsumer::default();
    let inputs = consumer.inputs();

    rusty_kode::discover_inputs(&paths, &mut reader, &mut consumer).unwrap_or_else(|error| {
        panic!("single stdin discovery should succeed ({evidence}): {error}")
    });

    assert!(
        accesses.get() > 0,
        "the explicit stdin route should consume the supplied reader ({evidence})"
    );
    let inputs = inputs.borrow();
    assert_eq!(
        inputs.len(),
        1,
        "stdin should be handed off once ({evidence})"
    );
    assert_eq!(
        inputs[0].display_name(),
        "-",
        "stdin identity should be exact ({evidence})"
    );
    assert_eq!(
        inputs[0].source(),
        source,
        "stdin source should be complete and unchanged ({evidence})"
    );
    drop(inputs);

    let mut failing_reader = FailingReader;
    let mut failing_consumer = RecordingMetricInputConsumer::default();
    let failed_inputs = failing_consumer.inputs();
    let error = rusty_kode::discover_inputs(&paths, &mut failing_reader, &mut failing_consumer)
        .expect_err("the injected stdin failure should be propagated");

    assert_eq!(
        error.kind(),
        io::ErrorKind::BrokenPipe,
        "the original I/O failure kind should be preserved ({evidence})"
    );
    assert!(
        failed_inputs.borrow().is_empty(),
        "a failed full-stream read must not hand off partial input ({evidence})"
    );
}
