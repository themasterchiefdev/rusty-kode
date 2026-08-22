mod support;

use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use support::python_shebang_evidence_context;

static FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct FixtureDirectory {
    path: PathBuf,
}

impl FixtureDirectory {
    fn new() -> Self {
        let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "rusty-kode-met-010-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir(&path).expect("MET-010 fixture directory should be created");
        Self { path }
    }

    fn write(&self, name: &str, contents: &[u8]) -> PathBuf {
        let path = self.path.join(name);
        fs::write(&path, contents).expect("MET-010 fixture should be written");
        path
    }

    fn missing(&self) -> PathBuf {
        self.path.join("missing")
    }
}

impl Drop for FixtureDirectory {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).expect("MET-010 fixtures should be cleaned up");
    }
}

#[test]
fn eligibility_depends_only_on_a_readable_python_shebang_first_line() {
    let evidence = python_shebang_evidence_context();
    let fixtures = FixtureDirectory::new();
    let cases = [
        (
            "env-python",
            b"#!/usr/bin/env python\nprint('accepted')\n".as_slice(),
            true,
        ),
        ("python3", b"#!/usr/bin/python3\n".as_slice(), true),
        ("uppercase", b"#!/usr/bin/env Python\n".as_slice(), false),
        ("no-prefix", b"python #!\n".as_slice(), false),
        ("no-python", b"#!/bin/sh\n".as_slice(), false),
        (
            "invalid-utf8",
            &[b'#', b'!', 0xff, b'p', b'y', b't', b'h', b'o', b'n'],
            false,
        ),
        ("empty", b"".as_slice(), false),
        ("leading-space", b" #!/usr/bin/python\n".as_slice(), false),
        (
            "later-line-only",
            b"#!/bin/sh\n#!/usr/bin/env python\n".as_slice(),
            false,
        ),
    ];

    for (name, contents, expected) in cases {
        let path = fixtures.write(name, contents);
        assert_eq!(
            rusty_kode::has_python_shebang(&path),
            expected,
            "first-line shebang eligibility mismatch for {name:?} ({evidence})"
        );
    }

    assert!(
        !rusty_kode::has_python_shebang(&fixtures.missing()),
        "a missing candidate should be ineligible ({evidence})"
    );

    let extensionless = fixtures.write("extensionless", b"#!/usr/bin/env python\n");
    assert!(
        rusty_kode::has_python_shebang(&extensionless),
        "an extensionless Python shebang should remain eligible ({evidence})"
    );
    assert!(
        !rusty_kode::is_python_filename(extensionless.as_os_str()),
        "shebang eligibility must remain separate from the .py rule ({evidence})"
    );

    let python_filename = fixtures.write("module.py", b"print('no shebang')\n");
    assert!(
        rusty_kode::is_python_filename(Path::new("module.py").as_os_str()),
        ".py eligibility must remain independent of shebang evidence ({evidence})"
    );
    assert!(
        !rusty_kode::has_python_shebang(&python_filename),
        "a .py filename alone must not satisfy the shebang rule ({evidence})"
    );
}
