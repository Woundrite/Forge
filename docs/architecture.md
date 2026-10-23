# Architecture

Forge is split into three logical layers:

1. **Shell integration** adapts invocation to PowerShell or CMD and applies
   validated shell-state actions.
2. **Core engine** loads configuration, discovers extensions, resolves
   commands, dispatches processes, and standardizes output and exit codes.
3. **Commands and extensions** provide built-in commands and independently
   installed external executables.

## Current modules

- `src/main.rs`: CLI argument parsing and process exit handling.
- `src/engine.rs`: command resolution, built-in commands, and plugin execution.
- `src/config.rs`: user configuration paths and TOML persistence.
- `src/plugins.rs`: manifest parsing, discovery, collision checks, and path
  validation.
- `src/protocol.rs`: versioned shell-state response types.
- `src/output.rs`: shared output format selection.
- `src/error.rs`: typed errors and exit-code mapping.

The dispatcher does not contain the implementation of every future command.
Commands should be added as focused modules or external plugins.

## Extension boundary

The first extension format is an external executable described by
`plugin.toml`. Forge validates the manifest and executable path, then passes
arguments directly to the child process. Native shared-library plugins are a
future versioned C-ABI feature and are not part of the current core.
