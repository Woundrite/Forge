# Plugin examples

This directory contains sample Forge plugins that exercise the externals-plugin
contract without requiring a core rebuild.

## Included examples

- `hello`: prints a greeting.
- `pick`: picks one option from a list.
- `dice`: rolls common dice notation such as `2d20` or `4d6kh3`.
- `tmpx`: creates a temporary directory and reports the path.
- `sysinfo`: displays the operating system and hardware summary.

## Layout

```text
plugins/
├── hello/
│   ├── plugin.toml
│   └── bin/
│       └── hello.ps1
├── pick/
│   ├── plugin.toml
│   └── bin/
│       └── pick.ps1
├── dice/
│   ├── plugin.toml
│   └── bin/
│       └── dice.ps1
└── README.md
```

## Register the directory

```powershell
forge config add-plugin-dir C:\path\to\Forge\plugins
forge plugin list
```

The command names exposed by Forge come from each plugin's `commands.*` table.
