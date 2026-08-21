mod support;

use std::{
    cell::{Cell, RefCell},
    ffi::OsString,
    io,
    rc::Rc,
};

#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;

use support::{combined_output, evidence_context, run_rusty_kode_without_arguments};

struct RecordingDispatch {
    calls: Rc<RefCell<Vec<Vec<OsString>>>>,
}

impl rusty_kode::Dispatch for RecordingDispatch {
    fn dispatch(self, arguments: Vec<OsString>) -> io::Result<()> {
        self.calls.borrow_mut().push(arguments);
        Ok(())
    }
}

#[test]
fn empty_invocation_displays_help_without_parser_failure_or_dispatch() {
    let output = run_rusty_kode_without_arguments();
    let combined = combined_output(&output);
    let normalized = combined.to_ascii_lowercase();
    let evidence = evidence_context();

    assert!(
        normalized.contains("rusty-kode") && normalized.contains("usage"),
        "empty invocation should display recognizable command help ({evidence}); output: {combined:?}"
    );
    for diagnostic in [
        "a subcommand is required",
        "required arguments were not provided",
        "missing command",
    ] {
        assert!(
            !normalized.contains(diagnostic),
            "empty invocation should not be a parser failure ({evidence}); output: {combined:?}"
        );
    }

    let dispatch_called = Cell::new(false);
    let mut application_output = Vec::new();
    rusty_kode::run(
        Vec::<OsString>::new(),
        |_| {
            dispatch_called.set(true);
            Ok(())
        },
        &mut application_output,
    )
    .expect("empty invocation help should render through the public runner");

    assert!(
        !dispatch_called.get(),
        "empty invocation must not invoke normal dispatch ({evidence})"
    );
    let application_output = String::from_utf8(application_output).expect("help should be UTF-8");
    let normalized_application_output = application_output.to_ascii_lowercase();
    assert!(
        normalized_application_output.contains("rusty-kode")
            && normalized_application_output.contains("usage"),
        "public runner should render shared top-level help ({evidence})"
    );
}

#[test]
fn non_empty_invocations_are_forwarded_unchanged_exactly_once() {
    let mut cases = vec![
        ("option-like", vec![OsString::from("--verbose")]),
        ("command-like", vec![OsString::from("analyze")]),
        ("invalid-looking", vec![OsString::from("???")]),
        (
            "ordered",
            vec![
                OsString::from("first"),
                OsString::from("--second"),
                OsString::from("third"),
            ],
        ),
    ];

    #[cfg(unix)]
    cases.push((
        "non-UTF-8",
        vec![OsString::from_vec(vec![b'a', 0x80, b'z'])],
    ));

    for (case, arguments) in cases {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let mut help_output = Vec::new();

        rusty_kode::run(
            arguments.clone(),
            RecordingDispatch {
                calls: Rc::clone(&calls),
            },
            &mut help_output,
        )
        .unwrap_or_else(|error| panic!("{case} invocation should be delegated: {error}"));

        assert_eq!(
            calls.borrow().as_slice(),
            &[arguments],
            "{case} invocation should be forwarded unchanged exactly once"
        );
        assert!(
            help_output.is_empty(),
            "{case} invocation must not be intercepted as MET-003 help"
        );
    }
}
