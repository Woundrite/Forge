# Plugin development

Forge plugins are external executables. A plugin package contains a
`plugin.toml` file and one or more executables below the same directory.

```toml
[plugin]
name = "hello"
version = "1.0.0"
description = "Example command"
api_version = 1

[commands.hello]
executable = "bin/hello.exe"
description = "Print a greeting"
```

Executable paths must be relative and cannot contain parent-directory
components. Forge rejects missing executables, invalid API versions, empty
command names, and duplicate command names.

Arguments are forwarded as a native argument list. Plugin stdout, stderr, and
exit status are preserved. Plugins should return `2` for invalid user input
and document any additional exit codes.

Plugins run with the user's permissions and are not sandboxed. Only install
plugins from sources you trust.
