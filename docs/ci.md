# Continuous integration

Forge uses GitHub Actions to validate changes before they are merged.

The workflow is defined in
[`.github/workflows/ci.yml`](../.github/workflows/ci.yml) and runs when:

- commits are pushed to `main`;
- a merge request targets `main`.

Each run checks the project on both Ubuntu and Windows with the stable Rust
toolchain. It:

1. checks Rust formatting with `cargo fmt --all -- --check`;
2. runs tests with `cargo test --all-targets --all-features`;
3. builds the release binary with `cargo build --release`.

The workflow uses least-privilege `contents: read` permissions, caches Cargo
dependencies and build output, and cancels older in-progress runs for the same
branch or merge request.

Do not bypass a failed required check. Fix the branch, push again, and wait for
the workflow to complete before merging.
