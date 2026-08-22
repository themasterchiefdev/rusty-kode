mod support;

use std::ffi::OsStr;

use support::python_extension_evidence_context;

#[test]
fn only_exact_lowercase_terminal_python_suffix_is_eligible() {
    let evidence = python_extension_evidence_context();
    let cases = [
        ("module.py", true),
        (".py", true),
        ("nested/path/module.py", true),
        ("module.py.bak", false),
        ("module.PY", false),
        ("module.Py", false),
        ("module.pY", false),
        ("module", false),
        ("contains.py.txt", false),
        ("module.py~", false),
        ("", false),
        ("p", false),
        ("py", false),
    ];

    for (filename, expected) in cases {
        assert_eq!(
            rusty_kode::is_python_filename(OsStr::new(filename)),
            expected,
            "eligibility should depend only on an exact lowercase terminal .py suffix: {filename:?} ({evidence})"
        );
    }
}
