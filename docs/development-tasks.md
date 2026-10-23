# Development task runner

Forge includes two equivalent task interfaces:

- `scripts/project.ps1` is the canonical Windows PowerShell runner.
- `Makefile` provides familiar targets when GNU Make or another compatible
  `make` implementation is available.

From the repository root:

| Task | PowerShell | Make |
| --- | --- | --- |
| Fetch dependencies | `.\scripts\project.ps1 setup` | `make setup` |
| Check formatting | `.\scripts\project.ps1 fmt` | `make fmt` |
| Type-check | `.\scripts\project.ps1 check` | `make check` |
| Run tests | `.\scripts\project.ps1 test` | `make test` |
| Debug build | `.\scripts\project.ps1 build` | `make build` |
| Release build | `.\scripts\project.ps1 release` | `make release` |
| Run Forge | `.\scripts\project.ps1 run -- version` | `make run ARGS="version"` |
| Remove build output | `.\scripts\project.ps1 clean` | `make clean` |
| Run all CI checks | `.\scripts\project.ps1 ci` | `make ci` |
| Install locally | `.\scripts\project.ps1 install` | `make install` |
| Uninstall locally | `.\scripts\project.ps1 uninstall` | `make uninstall` |

`clean` delegates to `cargo clean` and removes only Cargo's build output under
`target/`. It does not remove configuration, installed plugins, or user data.

The `run` task preserves argument boundaries after `--`. For example:

```powershell
.\scripts\project.ps1 run -- --output json version
```

The `ci` task runs formatting, `cargo check`, all tests, and a release build in
the same order as the GitHub Actions workflow.
