# rusty-kode

`rusty-kode` is an experimental Rust port of [Radon](https://github.com/rubik/radon), a Python package and command-line tool that calculates code metrics such as cyclomatic complexity, Halstead metrics, raw metrics, and maintainability index. Radon helps developers measure and inspect the complexity and maintainability of Python source code.

## Why this exists

The idea began while I was looking for a Python code-metrics library to improve the code-quality signals used by my AI projects. The Python community's recommendations led me to Radon; its latest upstream commit was made in October 2024, nearly two years before this experiment began.

More importantly, this repository is an experiment in using my **"mini software factory"** to build a real software port. The factory analyzed the pinned Radon repository, derived compatibility specifications and a dependency graph, and generated a backlog of 102 mapped user stories that are being implemented and verified in Rust, one vertical slice at a time.

This project is still at an early stage and is not ready for production use.

The implemented command currently:

- reports the application version when invoked with `--version`;
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

Report the application version with:

```sh
cargo run -- --version
```

## Project status

The canonical story order is recorded in [`docs/specs/IMPLEMENTATION-ORDER.md`](docs/specs/IMPLEMENTATION-ORDER.md). Completed features retain their specification, task checkpoints, tests, and acceptance evidence in `specs/`.

## Contributions

This is a publicly visible, owner-maintained repository. External code contributions and pull requests are not accepted. Security reports remain welcome through GitHub's private vulnerability-reporting channel; see [`SECURITY.md`](SECURITY.md).

## License

Copyright (C) 2026 Rajiv Yanamandra.

This project is licensed under the GNU General Public License, version 3 or any later version (`GPL-3.0-or-later`). See [`LICENSE`](LICENSE).

GPL obligations generally apply when covered or derivative works are distributed. Merely running an unmodified copy or making private modifications does not require public source disclosure. Consult the license text or qualified legal counsel for a particular use.
