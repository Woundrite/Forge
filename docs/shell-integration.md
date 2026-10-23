# Shell integration

The PowerShell adapter is `integrations/powershell/Forge.psm1`; the CMD
adapter is `integrations/cmd/forge.cmd`. Both forward ordinary commands to the
Forge executable.

Stateful commands will return a versioned JSON `ShellResponse`. Adapters must
validate the protocol version, action type, field sizes, and paths before
applying an action. Only documented actions such as changing directory or
setting an explicitly named environment variable may be supported.

Adapters must never evaluate arbitrary text returned by a plugin. In
particular, they must not use `Invoke-Expression`, `eval`, or equivalent
shell-string execution.
