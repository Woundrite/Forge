# Forge quick start

## Build

Install Rust, then run:

```powershell
cargo build --release
cargo test
```

## Core commands

```powershell
forge help
forge version
forge config path
forge config show
forge plugin list
```

Use `--output json` when another program needs machine-readable output.

## Add a plugin

Create a plugin directory with a `plugin.toml` manifest and an executable
inside that directory:

```text
plugins/
└── hello/
    ├── plugin.toml
    └── bin/
        └── hello.exe
```

The executable path in the manifest is relative to the plugin directory.
Absolute paths and parent-directory traversal are rejected.

```toml
[plugin]
name = "hello"
version = "1.0.0"
description = "A hello-world plugin"
api_version = 1

[commands.hello]
executable = "bin/hello.exe"
description = "Say hello"
```

Register and invoke it:

```powershell
forge config add-plugin-dir C:\path\to\plugins
forge plugin list
forge hello "Ada Lovelace"
```

Forge forwards each argument without shell-string concatenation, preserving
spaces and Unicode. The plugin's exit code is returned to the caller.

## Execution modes

- Default: run in the foreground and inherit standard streams.
- `--capture`: capture and forward output after the process exits.
- `--detached`: start the plugin without waiting; cannot be combined with
  `--wait` or `--capture`.
- `--newshell`: run the plugin through the configured platform shell.

`--detached` and `--wait` are rejected together to avoid ambiguous process
lifetime semantics.
