mod support;

use std::{ffi::OsString, io};

use support::{
    CountingReader, FailingReader, PanicOnRead, RecordingMetricInputConsumer,
    RecordingNonStandardInputDelegate, standard_input_evidence_context,
};

#[test]
fn single_standard_input_is_handed_off_atomically() {
    let evidence = standard_input_evidence_context();
    let source = "def answer():\n    return 42\n";
    let paths = [OsString::from("-")];
    let (mut reader, accesses) = CountingReader::new(source);
    let mut consumer = RecordingMetricInputConsumer::default();
    let inputs = consumer.inputs();
    let mut delegate = RecordingNonStandardInputDelegate::default();

    rusty_kode::discover_inputs(&paths, &mut reader, &mut consumer, &mut delegate).unwrap_or_else(
        |error| panic!("single stdin discovery should succeed ({evidence}): {error}"),
    );

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
    let error = rusty_kode::discover_inputs(
        &paths,
        &mut failing_reader,
        &mut failing_consumer,
        &mut delegate,
    )
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

#[test]
fn repeated_standard_input_tokens_collapse_to_one_handoff() {
    let evidence = standard_input_evidence_context();
    let cases = [
        (
            vec![OsString::from("-"), OsString::from("-")],
            "print('two tokens')\n",
        ),
        (
            vec![
                OsString::from("-"),
                OsString::from("-"),
                OsString::from("-"),
            ],
            "value = 3\nprint(value)\n",
        ),
    ];

    for (paths, source) in cases {
        let (mut reader, accesses) = CountingReader::new(source);
        let mut consumer = RecordingMetricInputConsumer::default();
        let inputs = consumer.inputs();
        let mut delegate = RecordingNonStandardInputDelegate::default();

        rusty_kode::discover_inputs(&paths, &mut reader, &mut consumer, &mut delegate)
            .unwrap_or_else(|error| {
                panic!("repeated stdin discovery should succeed ({evidence}): {error}")
            });

        assert!(
            accesses.get() > 0,
            "repeated tokens should consume the supplied reader ({evidence})"
        );
        let inputs = inputs.borrow();
        assert_eq!(
            inputs.len(),
            1,
            "repeated tokens should produce one handoff ({evidence})"
        );
        assert_eq!(
            inputs[0].display_name(),
            "-",
            "repeated-token stdin identity should be exact ({evidence})"
        );
        assert_eq!(
            inputs[0].source(),
            source,
            "repeated-token source should be complete and unchanged ({evidence})"
        );
    }
}

#[test]
fn non_standard_input_collections_bypass_stdin_and_delegate_unchanged() {
    let evidence = standard_input_evidence_context();
    let cases = [
        vec![OsString::from("-"), OsString::from("sample.py")],
        vec![
            OsString::from("first.py"),
            OsString::from("-"),
            OsString::from("second.py"),
            OsString::from("-"),
        ],
        vec![OsString::from("sample.py")],
        vec![],
    ];

    for paths in cases {
        let mut reader = PanicOnRead;
        let mut consumer = RecordingMetricInputConsumer::default();
        let inputs = consumer.inputs();
        let mut delegate = RecordingNonStandardInputDelegate::default();
        let delegated_paths = delegate.delegated_paths();

        rusty_kode::discover_inputs(&paths, &mut reader, &mut consumer, &mut delegate)
            .unwrap_or_else(|error| {
                panic!("non-standard input delegation should succeed ({evidence}): {error}")
            });

        assert!(
            inputs.borrow().is_empty(),
            "non-standard paths must not create stdin-derived inputs ({evidence})"
        );
        assert_eq!(
            delegated_paths.borrow().as_slice(),
            &[paths],
            "paths must be delegated once with order, multiplicity, and values unchanged ({evidence})"
        );
    }
}
