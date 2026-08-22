mod support;

use std::{
    cell::Cell,
    ffi::OsString,
    io::{self, Write},
};

use support::{run_rusty_kode_with_version, version_evidence_context};

#[test]
fn standalone_version_succeeds_repeatedly_with_current_version_on_stdout() {
    let evidence = version_evidence_context();

    for run in 1..=100 {
        let output = run_rusty_kode_with_version();

        assert!(
            output.status.success(),
            "standalone version run {run} should succeed ({evidence})"
        );
        assert!(
            output.stderr.is_empty(),
            "standalone version run {run} should not write stderr ({evidence})"
        );

        let stdout = String::from_utf8(output.stdout)
            .unwrap_or_else(|error| panic!("version output should be UTF-8 ({evidence}): {error}"));
        assert_eq!(
            stdout.lines().count(),
            1,
            "standalone version run {run} should write exactly one line ({evidence})"
        );
        assert!(
            stdout.ends_with('\n') && stdout.contains(env!("CARGO_PKG_VERSION")),
            "standalone version run {run} should report the current package version ({evidence}); stdout: {stdout:?}"
        );
    }
}

#[test]
fn standalone_version_uses_runner_output_without_dispatch() {
    let evidence = version_evidence_context();
    let dispatch_called = Cell::new(false);
    let mut output = Vec::new();

    rusty_kode::run(
        [OsString::from("--version")],
        |_| {
            dispatch_called.set(true);
            Ok(())
        },
        &mut output,
    )
    .unwrap_or_else(|error| panic!("standalone version should render ({evidence}): {error}"));

    assert!(
        !dispatch_called.get(),
        "standalone version must complete before dispatch ({evidence})"
    );
    let output = String::from_utf8(output).expect("runner version output should be UTF-8");
    assert_eq!(
        output.lines().count(),
        1,
        "version output should be one line"
    );
    assert!(output.ends_with('\n'));
    assert!(output.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn non_standalone_version_arguments_remain_delegated_unchanged() {
    let arguments = vec![OsString::from("--version"), OsString::from("source.py")];
    let delegated = Cell::new(false);
    let mut output = Vec::new();

    rusty_kode::run(
        arguments.clone(),
        |received| {
            delegated.set(true);
            assert_eq!(received, arguments);
            Ok(())
        },
        &mut output,
    )
    .expect("non-standalone arguments should remain on the existing path");

    assert!(delegated.get());
    assert!(output.is_empty());
}

struct FailingWriter;

impl Write for FailingWriter {
    fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(
            io::ErrorKind::BrokenPipe,
            "injected version-output failure",
        ))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn standalone_version_propagates_output_failures_without_dispatch() {
    let dispatch_called = Cell::new(false);
    let error = rusty_kode::run(
        [OsString::from("--version")],
        |_| {
            dispatch_called.set(true);
            Ok(())
        },
        &mut FailingWriter,
    )
    .expect_err("version output failure should be propagated");

    assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
    assert!(!dispatch_called.get());
}
