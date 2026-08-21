mod support;

use std::{cell::Cell, ffi::OsString};

use support::{combined_output, evidence_context, run_rusty_kode_without_arguments};

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
