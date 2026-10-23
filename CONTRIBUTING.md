# Contributing to Forge

Thank you for contributing. Forge is developed through small, reviewable
changes that preserve the core's stability and security boundaries.

## Before starting

1. Read the [README](README.md) and [product specification](docs/specification.md).
2. Check existing branches and merge requests to avoid duplicate work.
3. Open or reference an issue for non-trivial work.
4. Keep the change focused. Separate unrelated cleanup into another branch.

## Branch policy

All work must happen on a separate branch and enter `main` through a merge
request. Direct pushes to `main` are not part of the project workflow.

Use one of these prefixes:

- `feature/<short-name>` for new functionality.
- `fix/<short-name>` for bug fixes.
- `docs/<short-name>` for documentation-only changes.
- `refactor/<short-name>` for behavior-preserving restructuring.
- `test/<short-name>` for test changes.
- `chore/<short-name>` for maintenance and tooling.

Example:

```powershell
git switch main
git pull --ff-only
git switch -c feature/project-registry
```

## Local checks

Run the checks relevant to the change before opening a merge request:

```powershell
cargo fmt -- --check
cargo test
cargo build
```

These same checks run automatically on Ubuntu and Windows for every push to
`main` and every merge request targeting `main`. The workflow is documented in
[`docs/ci.md`](docs/ci.md).

Feature branches should include tests for new behavior and error paths. Changes
to command output, exit codes, configuration, plugin manifests, or shell actions
must update the related documentation.

## Merge request requirements

Every merge request should:

- Explain the problem and the chosen solution.
- Link the issue or specification requirement when one exists.
- List user-visible behavior changes.
- Include tests and their results.
- Call out platform-specific behavior, especially Windows shell behavior.
- Avoid unrelated formatting or generated files.
- Update documentation for public interfaces.

Reviewers should verify correctness, security, backward compatibility, and
whether the change belongs in the core or an extension.

## Commit messages

Use concise conventional prefixes such as `feat:`, `fix:`, `docs:`, `test:`,
`refactor:`, and `chore:`. Keep commits cohesive and avoid committing build
artifacts or local configuration.

## Plugin contributions

Plugins must not rely on shell-string concatenation for user input. Manifests
must use the supported API version, keep executable paths within the plugin
directory, and document required permissions and external resources.

## Reporting security issues

Do not disclose an exploitable vulnerability in a public issue. Contact the
maintainers privately with reproduction details and affected versions.
