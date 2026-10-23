# Installation and removal

Forge provides Windows PowerShell and CMD entry points in `scripts/`.

## Install

From the repository root:

```powershell
.\scripts\install.ps1 -AddToPath -InstallShellIntegration
```

The installer:

1. builds `target\release\forge.exe`;
2. installs the executable and sample plugins under
   `%LOCALAPPDATA%\Forge`;
3. registers the installed plugin directory in Forge configuration;
4. optionally adds the install directory to the user `PATH`;
5. optionally installs the PowerShell module;
6. writes a managed-install marker used by the uninstaller.

Use `.\scripts\install.cmd` when starting from CMD. Use `-InstallDir` to
choose another installation directory and `-SkipBuild` when a release binary
already exists.

## Uninstall

```powershell
.\scripts\uninstall.ps1
```

The uninstaller removes only the files and plugin directories recorded by the
managed-install marker. It removes the PATH entry and PowerShell module only
when this installation added them.

Configuration and user data are preserved by default. To explicitly remove
the Forge data directory as well:

```powershell
.\scripts\uninstall.ps1 -PurgeData
```

Use `.\scripts\uninstall.cmd` from CMD.
