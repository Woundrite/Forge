# Forge CLI

Forge is a fast, extensible command-line toolkit for developer workflows,
system inspection, file operations, automation, and everyday productivity.
It is built in Rust with a small core and an external-plugin model so new
commands can be installed without rebuilding the engine.

> The project is in active foundation development. The current release
> provides the core dispatcher, configuration, plugin discovery, process
> execution, output modes, and shell-integration foundations.

## What Forge provides

- Native command dispatch with stable exit codes.
- Human-readable, plain-text, and JSON output modes.
- TOML-based local configuration.
- External executable plugins with validated manifests.
- Safe plugin executable path validation and argument forwarding.
- Foreground, detached, captured-output, and new-shell execution modes.
- Versioned shell-state protocol foundations for PowerShell and CMD.

## Quick start

### Requirements

- Rust stable toolchain with Cargo.
- Windows is the initial supported platform.
- PowerShell or CMD for shell integration.

### Build and test

```powershell
cargo build
cargo test
```

Run the CLI from the repository with Cargo:

```powershell
cargo run -- help
cargo run -- version
cargo run -- --output json version
cargo run -- plugin list
```

For a complete walkthrough, see [`docs/quickstart.md`](docs/quickstart.md).

## Project layout

```text
Forge/
├── src/                         # Rust engine and command dispatch
├── integrations/
│   ├── powershell/              # PowerShell adapter
│   └── cmd/                     # CMD adapter
├── docs/                        # User, contributor, and architecture docs
├── Cargo.toml                   # Rust package manifest
├── Cargo.lock                   # Reproducible dependency versions
└── forge_cli_specification_v1_1.md
                                # Product specification
```

## Documentation

- [Quick start](docs/quickstart.md)
- [Architecture](docs/architecture.md)
- [Command reference](docs/commands.md)
- [Plugin development](docs/plugin-development.md)
- [Shell integration](docs/shell-integration.md)
- [Security model](docs/security.md)
- [Continuous integration](docs/ci.md)
- [Contributing](CONTRIBUTING.md)
- [Product specification](docs/specification.md)

## Development workflow

Every feature, bug fix, or documentation change must be developed on its own
branch and merged through a merge request. Do not commit feature work directly
to `main`.

```powershell
git switch main
git pull
git switch -c feature/plugin-installation
# make focused changes
cargo fmt
cargo test
git add .
git commit -m "feat: add plugin installation"
git push -u origin feature/plugin-installation
```

Open a merge request from the branch, complete the checklist in the request
template, and merge only after review and passing checks. See
[`CONTRIBUTING.md`](CONTRIBUTING.md) for the complete policy.

## License

The project license has not yet been selected. Do not redistribute Forge or
contribute third-party code until a license is added by the maintainers.
