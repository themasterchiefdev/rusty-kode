# rusty-kode

`rusty-kode` is an early-stage, Python-first code metrics analyzer written in Rust. The project is being delivered story by story from compatibility specifications and is not yet ready for production use.

The implemented command currently:

- displays top-level help when invoked without arguments;
- preserves non-empty argument sequences for downstream dispatch.

## Development

Development is performed in the repository's dev container. Open the repository in a dev-container-compatible editor, then verify changes with:

```sh
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

The same checks run in GitHub Actions for every pull request and every push to `main`. The `main` branch requires the `Rust` check to pass before a pull request can merge.

Run the current executable with:

```sh
cargo run
```

## Project status

The canonical story order is recorded in [`docs/specs/IMPLEMENTATION-ORDER.md`](docs/specs/IMPLEMENTATION-ORDER.md). Completed features retain their specification, task checkpoints, tests, and acceptance evidence in `specs/`.

## Contributions

This is a publicly visible, owner-maintained repository. External code contributions and pull requests are not accepted. Security reports remain welcome through GitHub's private vulnerability-reporting channel; see [`SECURITY.md`](SECURITY.md).

## License

Copyright (C) 2026 Rajiv Yanamandra.

This project is licensed under the GNU General Public License, version 3 or any later version (`GPL-3.0-or-later`). See [`LICENSE`](LICENSE).

GPL obligations generally apply when covered or derivative works are distributed. Merely running an unmodified copy or making private modifications does not require public source disclosure. Consult the license text or qualified legal counsel for a particular use.
