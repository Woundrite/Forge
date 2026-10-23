# Command reference

## Core commands

### `forge help [command]`

Displays global or command-specific help.

### `forge version`

Displays the engine, command API, and shell protocol versions.

### `forge config`

- `show`: display the current configuration.
- `path`: display the user configuration file path.
- `add-plugin-dir <path>`: register a plugin discovery directory.

### `forge plugin list`

Lists commands discovered from configured plugin directories.

## Global options

- `--output human|plain|json`: select output format.
- `--detached`: start a plugin without waiting.
- `--newshell`: execute a plugin through a new shell.
- `--wait`: wait for a foreground child process.
- `--capture`: capture and forward child output after completion.

`--detached` cannot be combined with `--wait` or `--capture`.
