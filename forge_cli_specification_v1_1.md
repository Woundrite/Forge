# Forge CLI

## Product Requirements and Technical Specification

**Document version:** 1.1\
**Update:** Expanded command catalogue and added an extension-first
implementation plan\
**Status:** Proposed baseline specification\
**Project type:** Native, modular command-line utility toolkit\
**Initial platform:** Windows\
**Initial shells:** PowerShell and CMD\
**Core language:** Rust (recommended; C++ remains a viable alternative)\
**Working name:** Forge CLI

------------------------------------------------------------------------

## 1. Executive Summary

Forge CLI is a fast, compiled, extensible command-line toolkit for
everyday productivity, developer workflows, system inspection, file
operations, automation, and recreation.

It should feel like a native extension of the user's shell while keeping
a small, stable core. Users must be able to add commands, subcommands,
and plugins without recompiling the engine. The extension model should
resemble game modding: the engine provides stable interfaces, while
independently installed packages add capabilities.

The project will begin on Windows with PowerShell and CMD integration,
but its architecture must allow future support for Linux, macOS, Bash,
and Zsh.

### 1.1 Goals

-   Fast startup and low idle resource usage.
-   Native filesystem and operating-system integration where useful.
-   Commands usable from the current shell by default.
-   Optional detached execution and launching commands in a new shell.
-   Runtime plugin discovery and command registration without rebuilding
    the engine.
-   A clean, documented interface for built-in commands, external
    executables, scripts, and future native plugins.
-   Local-first configuration and persistence.
-   Safe defaults, useful diagnostics, and testable behavior.
-   Cross-platform architecture without forcing the initial release to
    implement every platform.

### 1.2 Non-goals

Forge CLI is not intended to replace the user's shell, operating system,
package manager, IDE, version-control system, or full automation
platform. It should complement existing tools and avoid duplicating
their entire feature sets.

------------------------------------------------------------------------

## 2. Product Principles

1.  **Small core:** The engine should focus on dispatch, discovery,
    configuration, process handling, shared services, and shell
    integration.
2.  **Extensible by design:** New commands and subcommands must not
    require changes to the dispatcher or recompilation of the core.
3.  **Fast by measurement:** Performance targets must be verified with
    repeatable benchmarks rather than assumed from the implementation
    language.
4.  **Shell-aware:** The shell integration layer must handle operations
    that change the current shell's state.
5.  **Local-first:** Personal data remains local unless a feature
    explicitly requires network access.
6.  **Safe by default:** Destructive operations and untrusted extensions
    require appropriate safeguards.
7.  **Portable interfaces:** Avoid coupling plugin contracts to one
    operating system, shell, or programming language.
8.  **Optional complexity:** AI integrations, background services, and
    interactive terminal interfaces must remain optional.
9.  **Observable behavior:** Commands should have predictable output,
    exit codes, error messages, and logging.
10. **User-owned configuration:** Upgrading the executable must not
    overwrite personal data or user configuration.

------------------------------------------------------------------------

## 3. Platform and Technology Decisions

  ---------------------------------------------------------------------
  Area                               Decision
  ---------------------------------- ----------------------------------
  Initial operating system           Windows

  Initial shells                     PowerShell and CMD

  Future operating systems           Linux and macOS

  Future shells                      Bash, Zsh, and others where
                                     practical

  Core language                      Rust recommended; C++ acceptable
                                     if project priorities favor it

  Build system                       Cargo for Rust; CMake for C++

  Command execution                  Compiled native executable

  First plugin format                External executable plus manifest

  Later plugin format                Native shared library through a
                                     versioned C-compatible ABI

  Optional extension format          PowerShell/CMD scripts initially
                                     where useful

  Configuration                      TOML or another documented,
                                     portable text format

  Structured persistence             SQLite when a feature needs a
                                     local database

  Output                             Human-readable by default; plain
                                     text and JSON modes where relevant

  Background service                 Not required by default

  Distribution                       Windows executable and shell
                                     integration files initially
  ---------------------------------------------------------------------

### 3.1 Language rationale

Rust is the recommended default for the core because it combines native
performance with strong compile-time memory-safety guarantees. C++
remains suitable, particularly if direct Windows API work or C++-first
plugin development is a priority.

The architecture must not rely on Rust's ordinary native ABI or C++
compiler-specific ABI as a stable cross-version plugin contract. Native
plugins will use an explicitly versioned C-compatible interface.
External executable plugins remain language-independent.

### 3.2 Native performance caveat

Compiled code does not automatically make every operation fast.
Algorithm choice, filesystem APIs, process creation, antivirus scanning,
storage devices, and OS behavior all affect performance. The project
must measure engine overhead separately from the work a command
performs.

------------------------------------------------------------------------

## 4. System Architecture

Forge CLI is organized into three main layers:

1.  **Shell integration:** Adapts command invocation to the current
    shell and applies shell-state changes.
2.  **Core engine:** Parses arguments, resolves commands, loads
    configuration, discovers plugins, dispatches execution, and
    standardizes results.
3.  **Commands and extensions:** Built-in commands, external
    executables, scripts, and later native plugins.

Logical architecture:

``` text
User
 |
 +-- PowerShell / CMD
 |       |
 |       +-- Shell integration adapter
 |                 |
 |                 +-- Forge CLI executable
 |                           |
 |                           +-- Argument parser
 |                           +-- Dispatcher
 |                           +-- Command registry
 |                           +-- Plugin discovery
 |                           +-- Configuration manager
 |                           +-- Process manager
 |                           +-- Output and error handling
 |                           +-- Shared host services
 |                                      |
 |                   +------------------+------------------+
 |                   |                  |                  |
 |              Built-in commands  External plugins  Native plugins*
 |                   |                  |                  |
 |                   +------------------+------------------+
 |                                      |
 |                         Filesystem / OS APIs / SQLite
 |
 * Native shared-library plugins are a later phase.
```

These are logical components; they do not all need to be separate
processes or packages in the first release.

### 4.1 Core engine responsibilities

-   Parse arguments and options.
-   Resolve command names, aliases, and subcommands.
-   Discover and validate extension manifests.
-   Dispatch to built-in or extension implementations.
-   Load only configuration and services needed by the requested
    command.
-   Standardize output, diagnostics, and exit codes.
-   Manage child processes and execution modes.
-   Expose documented host services to supported plugin types.
-   Return structured shell actions to the shell adapter when a command
    must modify shell state.

The dispatcher must not contain the implementation of every command.

### 4.2 Command registry

A registry entry should conceptually include:

``` text
Command:
    canonical_name
    aliases
    description
    owner_plugin
    implementation_type
    executable_or_entry_point
    argument_schema
    required_capabilities
    command_api_version
```

Commands and subcommands must be registered deterministically. Duplicate
registrations must produce explicit errors rather than silently
replacing an existing implementation.

A plugin should be able to register a group of commands atomically: if a
conflict occurs, none of that group's registrations should be committed.

### 4.3 Command precedence

Recommended resolution order:

1.  Reserved core commands.
2.  Registered plugin commands.
3.  Explicitly configured user aliases.
4.  Optional external-command fallback, if enabled.

Plugins must not be able to shadow reserved core commands. Top-level
aliases are permitted only when conflict-free or explicitly approved by
the user.

------------------------------------------------------------------------

## 5. Command Interface and Conventions

### 5.1 General syntax

``` text
forge <command> [subcommand] [arguments] [options]
```

Examples:

``` powershell
forge help
forge version
forge proj list
forge proj open ingestion-engine
forge mkproj cpp packet-parser
forge ports 8000
forge bench run --runs 100 -- ./program
forge todo add "Complete project documentation"
forge fortune programming
```

The exact syntax for forwarding arguments to a child program must be
documented. A `--` separator should be supported where it avoids
ambiguity between Forge options and child-process arguments.

### 5.2 Command requirements

Every command should provide:

-   Canonical name and description.
-   Usage and help.
-   Argument and option validation.
-   Consistent output and diagnostics.
-   Documented exit statuses.
-   Optional configuration schema.
-   Dependency declarations, if applicable.
-   Tests for normal and error paths.

### 5.3 Help

The following should work:

``` powershell
forge help
forge help proj
forge proj --help
```

Help should include purpose, syntax, arguments, options, subcommands,
examples, and any required dependencies.

### 5.4 Output formats

Commands should use a shared output interface and support, where
appropriate:

-   **Human-readable:** tables, concise messages, and optional terminal
    styling.
-   **Plain text:** minimal formatting for pipelines and logs.
-   **JSON:** machine-readable output for scripting and integrations.

Output formatting must be separate from business logic. JSON output must
not be mixed with human-readable status text on standard output;
diagnostics belong on standard error.

### 5.5 Exit codes and errors

Use consistent exit-code conventions. At minimum, distinguish success,
invalid usage, command failure, missing dependency, and internal error.
Preserve child-process exit codes when the command is explicitly acting
as a process wrapper, unless a documented mapping is necessary.

Errors should state what failed and offer a corrective action when
practical. Never print secrets or dump the full environment as part of
routine diagnostics.

------------------------------------------------------------------------

## 6. Plugin and Extension System

### 6.1 Requirement

Users must be able to install or register extensions and add commands or
subcommands without recompiling the Forge CLI executable.

The system should support multiple implementation models because
convenience, isolation, and performance differ by extension type.

### 6.2 Extension types

  -------------------------------------------------------------------------
  Type             Initial status   Purpose                Trade-offs
  ---------------- ---------------- ---------------------- ----------------
  External         Required for     General-purpose,       Extra process
  executable       first plugin     language-independent   startup; strong
                   milestone        plugins                fault isolation
                                                           compared with
                                                           in-process code

  Script plugin    Optional early   Quick automation and   Interpreter
                   feature          personal commands      startup and
                                                           shell-specific
                                                           behavior

  Native shared    Later phase      Low-overhead,          ABI complexity;
  library                           high-performance       a fault can
                                    extensions             crash the engine

  Built-in command Initial release  Essential, stable      Requires
                                    commands               rebuilding to
                                                           change built-in
                                                           implementation
  -------------------------------------------------------------------------

### 6.3 External executable plugins

A plugin is an independently built executable discovered by Forge and
invoked with the requested arguments.

Example package:

``` text
plugins/
└── weather/
    ├── plugin.toml
    ├── bin/
    │   └── weather.exe
    └── README.md
```

Illustrative manifest:

``` toml
[plugin]
name = "weather"
version = "1.0.0"
description = "Weather information for the terminal"
api_version = 1

[commands.weather]
executable = "bin/weather.exe"
description = "Show current weather"
```

The engine should resolve the executable relative to the validated
plugin directory, forward arguments safely, and preserve standard
output, standard error, and exit status according to the plugin
contract.

External plugins may be written in Rust, C++, Go, or another language
capable of producing a compatible executable.

### 6.4 Script plugins

Script extensions may call PowerShell or another explicitly configured
interpreter. They are intended for convenience and automation, not as
the default for performance-critical commands.

The engine must not concatenate arbitrary user input into a shell
command string when it can pass arguments directly through a process
API. Script execution behavior and quoting rules must be documented per
interpreter.

### 6.5 Native shared-library plugins

A later release may support DLLs on Windows and shared libraries on
other operating systems. The interface must use a narrow, versioned C
ABI with explicit structure layouts, data types, error handling, and
memory-ownership rules.

Requirements:

-   Check the plugin API version before activation.
-   Validate architecture and platform compatibility.
-   Do not expose compiler-specific C++ classes as the permanent ABI.
-   Document which side allocates and frees every buffer.
-   Handle initialization failures and shutdown consistently.
-   Isolate plugin failures where feasible.

Native plugins execute inside the engine process and therefore cannot be
treated as safely sandboxed. A crash or memory-corruption bug may
terminate the engine.

### 6.6 Plugin discovery and caching

-   Discover plugins from configured plugin directories.
-   Validate manifests before registering commands.
-   Do not load every plugin executable or native library at startup.
-   Avoid rescanning all plugin files on every invocation when a safe
    metadata cache is available.
-   Provide cache invalidation when plugins are installed, removed,
    enabled, or disabled.
-   Never perform network requests during ordinary command discovery.
-   Keep discovery deterministic and report invalid manifests clearly.

### 6.7 Plugin management commands

Proposed interface:

``` powershell
forge plugin list
forge plugin info weather
forge plugin install C:\Dev\forge-plugins\weather
forge plugin validate weather
forge plugin disable weather
forge plugin enable weather
forge plugin remove weather
```

The first release may support local-directory installation only. A
remote repository or marketplace is explicitly out of scope for the
initial release.

Installation should validate the manifest, compatibility, files, and
command conflicts before committing changes. Failed installation must
not leave a partially registered plugin without a clear recovery path.

### 6.8 Plugin security

A plugin is executable code. External executables can usually access
resources available to the current user, and native plugins run with the
engine's privileges. Manifest-declared capabilities improve transparency
but do not sandbox code.

Minimum requirements:

-   Treat downloaded plugins as untrusted until verified.
-   Require explicit user action to install or enable extensions.
-   Display plugin origin and requested capabilities where available.
-   Never auto-run newly discovered code solely because it exists in a
    folder.
-   Avoid silently replacing existing plugin files.
-   Plan package hashes and trusted publisher signatures for remote
    distribution.
-   Do not claim that a plugin is sandboxed unless a real OS-enforced
    isolation mechanism is used.

------------------------------------------------------------------------

## 7. Shell Integration and Current-Shell Behavior

### 7.1 Fundamental constraint

A child executable cannot directly change the working directory or
environment of its parent shell. Calling a Windows directory-change API
inside `forge.exe` changes only the executable's own process state.

Therefore, ordinary commands can be handled by the executable, but
shell-state changes require a PowerShell or CMD adapter that applies a
structured action in the current shell.

### 7.2 Process commands versus shell-state commands

**Process commands** run and return results:

``` powershell
forge ports
forge sysinfo
forge bench run -- ./program
```

**Shell-state commands** modify the calling shell:

``` powershell
forge proj open ingestion-engine
forge env activate MLENV
forge directory previous
```

The engine should resolve the requested operation and return a
structured action. The shell adapter validates and applies that action.
The protocol must not depend on parsing human-readable output.

### 7.3 PowerShell adapter

Provide a PowerShell module that defines a `forge` function and safely
forwards arguments to the executable.

For shell-state commands, the module should request a machine-readable
action from the engine, validate the action and its fields, and then
apply supported changes using native PowerShell commands such as
`Set-Location`.

The adapter should not evaluate arbitrary code returned by plugins. Only
documented, allowlisted shell actions should be accepted.

### 7.4 CMD adapter

Provide a CMD-compatible batch adapter for ordinary command execution
and supported shell-state actions. The adapter must account for CMD's
distinct quoting, escaping, and environment semantics.

A batch file can change the active CMD session's directory when called
in the same session, but a child executable cannot directly set the
caller's environment. The protocol must therefore return validated
actions for the batch adapter to apply.

### 7.5 Shell-state protocol

The protocol should define a versioned, machine-readable response with:

-   Protocol version.
-   Action type.
-   Action arguments.
-   Optional diagnostic message.
-   Validation rules and size limits.

Initially supported actions should be limited to a small allowlist, such
as changing directory and setting or removing an explicitly named
environment variable. Unsupported actions must fail safely.

Do not use `eval`, arbitrary PowerShell script evaluation, or equivalent
execution of untrusted output.

### 7.6 Current-shell default

The default command behavior is to run in the current terminal session.
Shell-state operations are applied by the integration adapter. Commands
that do not need to alter shell state should execute normally and
display output in the current terminal.

------------------------------------------------------------------------

## 8. Execution Modes

The CLI should support clearly defined execution modes rather than
treating every flag as an independent switch.

  ---------------------------------------------------------------------
  Mode / option                      Behavior
  ---------------------------------- ----------------------------------
  Default                            Run normally in the current
                                     terminal; apply supported shell
                                     actions through the adapter

  `--detached`                       Start a child process
                                     independently of the foreground
                                     command

  `--newshell`                       Open a new shell window and run
                                     the requested command there

  `--wait`                           Wait for the launched child
                                     process to finish

  `--capture`                        Capture child output where
                                     supported
  ---------------------------------------------------------------------

Examples:

``` powershell
forge ports
forge --detached workflow run long-analysis
forge --newshell proj open ingestion-engine
forge --wait bench run -- ./program
```

The exact command grammar should be specified during implementation so
global execution flags cannot be confused with arguments intended for a
plugin.

### 8.1 Mode rules

-   `--detached` and `--wait` are incompatible unless a future design
    explicitly defines a useful combined behavior.
-   `--capture` requires a process mode that supports captured output.
-   `--newshell` must select the configured or explicitly requested
    shell.
-   A new shell must apply directory changes inside that new shell, not
    attempt to modify the parent shell.
-   Document working-directory inheritance, environment inheritance,
    output routing, process lifetime, and exit-code behavior for each
    mode.
-   Reject ambiguous or incompatible option combinations with a clear
    error.

------------------------------------------------------------------------

## 9. Initial Command Catalogue

The initial command catalogue should favor useful, bounded features.
Commands can be built in or shipped as plugins depending on stability
and dependency needs.

### 9.1 Core commands

  Command     Purpose                                    Example
  ----------- ------------------------------------------ ------------------------
  `help`      Display global or command-specific help    `forge help proj`
  `version`   Display engine and API versions            `forge version`
  `config`    Inspect and update configuration           `forge config show`
  `plugin`    Discover and manage extensions             `forge plugin list`
  ump\` Ju    mp to frequently used directories \`f      orge jump engine\`
  nvx\` Sw    itch named project environment sets \`fo   rge envx use staging\`

### 9.2 Developer tools

  ----------------------------------------------------------------------------------
  Command            Purpose            Example
  ------------------ ------------------ --------------------------------------------
  `proj`             Register, search,  `forge proj open ingestion-engine`
                     list, and open     
                     projects           

  `mkproj`           Create projects    `forge mkproj rust cli`
                     from templates     

  `bench`            Benchmark external `forge bench run --runs 50 -- ./program`
                     programs           

  `context`          Summarize a        `forge context . --depth 3`
                     project's          
                     structure and      
                     status             

  `watchx`           Rerun a command    `forge watchx -- cargo test`
                     when selected      
                     files change       

  `workflow`         Execute named,     `forge workflow run daily-checks`
                     reusable command   
                     sequences          

  `gitx`             Summarize Git      oss `forge gitx`
                     status acr         
                     registered         
                     projects           

  `snip`             Search a personal  `forge snip search "retry loop"`
                     code snippet       
                     library            

  `jsonx`            Pretty-print,      diff
                     query, and JSON    `forge jsonx get .items[0].name data.json`
                     input              

  `diffdir`          Compare directory  ts `forge diffdir old/ new/`
                     conten             

  `timeit`           Record command     `forge timeit -- cargo build`
                     duration           
  ----------------------------------------------------------------------------------

### 9.3 System and filesystem tools

  ------------------------------------------------------------------------------
  Command             Purpose             Example
  ------------------- ------------------- --------------------------------------
  `sysinfo`           Display selected    `forge sysinfo`
                      system and          
                      development         
                      information         

  `ports`             Inspect listening   `forge ports 8000`
                      ports and owning    
                      processes where     
                      permitted           

  `killport`          Identify and        `forge killport 8000`
                      optionally          
                      terminate a port    
                      owner               

  `tmpx`              Create temporary    `forge tmpx --ttl 2h`
                      scratch directories 

  `rename`            Preview and apply   `forge rename "*.jpeg" "{n:03}.jpg"`
                      safe batch renames  

  `qr`                Render text/URL QR  `forge qr "https://example.com"`
                      codes               

  `serve`             Serve local static  `forge serve . --qr`
                      files               

  `netcheck`          Diagnose DNS,       `forge netcheck example.com`
                      connectivity, and   
                      HTTPS access        

  `findx`             Search files by     `forge findx --root . --name "*.rs"`
                      name, type, or      
                      supported content   
                      pattern             

  `dux`               Summarize directory `forge dux .`
                      sizes               

  `hashx`             Calculate or verify `forge hashx verify checksums.txt`
                      file hashes         

  `cleanx`            Preview and safely  `forge cleanx --dry-run`
                      remove configured   
                      generated artifacts 
  ------------------------------------------------------------------------------

### 9.4 Productivity tools

  ------------------------------------------------------------------------------------------
  Command         Purpose         Example
  --------------- --------------- ----------------------------------------------------------
  `todo`          Create and      `forge todo add "Write tests"`
                  manage tasks    

  `note`          Store and       `forge note search "connection pool"`
                  search quick    
                  notes           

  `daily`         Display tasks,  `forge daily`
                  reminders, and  
                  focus summary   

  `focus`         Track focus     `forge focus start 25m`
                  sessions        

  `remind`        Manage          `forge remind add "Submit form" --at "2026-10-22 09:00"`
                  reminders       

  `journal`       Create          `forge journal add "Investigated startup time"`
                  timestamped     
                  daily entries   

  `clip`          Save snippets   t `forge clip save deploy-cmd`
                  and inspec      
                  clipboard       
                  history         

  `ago` / `until` Calculate       `forge until 2026-12-01` `forge ago 90d`
                  relative dates  
  ------------------------------------------------------------------------------------------

### 9.5 Recreational and optional commands

  ----------------------------------------------------------------------------
  Command              Purpose              Example
  -------------------- -------------------- ----------------------------------
  `fortune`            Display jokes,       `forge fortune programming`
                       quotations, facts,   
                       or programming humor 

  `pet`                Maintain a virtual   `forge pet status`
                       terminal companion   

  `snake`              Play a terminal game `forge snake`

  `quiz`               Run short quizzes or `forge quiz programming`
                       trivia               

  `dice`               Roll dice or choose  `forge dice 4d6kh3`
                       randomly             

  `pick`               Pick from supplied   `forge pick pizza sushi tacos`
                       options              

  `wordle`             Play Wordle with a   `forge wordle`
                       local streak tracker 

  `life` / `matrix` /  Terminal             `forge life`
  `pipes`              screensavers         

  `ask`                Optional AI-assisted `forge ask "Explain this error"`
                       question answering   

  `review`             Optional Git-diff    `forge review --staged`
                       review               
  ----------------------------------------------------------------------------

The initial release does not need to implement every listed command. The
catalogue is a backlog, not a release blocker.

------------------------------------------------------------------------

## 10. Initial Command Requirements

### 10.1 `proj` --- Project manager

Required features:

-   Add a project by name and path.
-   List and search registered projects.
-   Open a project in the current shell.
-   Display project metadata.
-   Track recent projects.
-   Detect missing directories.
-   Reject duplicate names unless the user explicitly resolves the
    conflict.

Acceptance criteria:

-   Registry changes persist across runs.
-   Paths are normalized according to platform rules.
-   Shell navigation works through the integration adapter.
-   Missing paths produce useful errors and do not corrupt the registry.

### 10.2 `mkproj` --- Project scaffolder

Required features:

-   List templates.
-   Create a project from a template.
-   Substitute validated template variables.
-   Support user-defined templates.
-   Optionally initialize version control.
-   Refuse to overwrite existing files without explicit permission.

Acceptance criteria:

-   Templates are stored outside the core executable.
-   A failed generation reports any partial output and provides recovery
    guidance.
-   Generated output is deterministic for a given template and set of
    variables where practical.

### 10.3 `ports` --- Port inspector

Required features:

-   List listening ports.
-   Filter by port and protocol.
-   Show process identifiers and names when available.
-   Explain when permissions prevent process identification.

Acceptance criteria:

-   Distinguish listening sockets from other connections.
-   Do not terminate processes.
-   Avoid requiring elevation unless the requested information genuinely
    requires it.

### 10.4 `bench` --- Benchmark runner

Required features:

-   Configurable run count and warm-up count.
-   Minimum, maximum, mean, and median duration.
-   Percentiles when sample count is adequate.
-   Exit-code and timeout tracking.
-   JSON and CSV export where practical.

Acceptance criteria:

-   Launch programs with argument-safe process APIs.
-   Record failures rather than silently counting them as successes.
-   Document timing methodology and limitations.
-   Separate child-process startup cost from other measurements when the
    selected mode supports it.

### 10.5 `todo`, `note`, and `daily`

Required features:

-   Persistent task and note storage.
-   Task priorities, optional due dates, completion, and search.
-   Note titles, content, and optional tags.
-   A dashboard that aggregates information through shared interfaces.

Acceptance criteria:

-   Stable identifiers for tasks and notes.
-   Unicode text support.
-   Useful behavior when no data exists.
-   Exportable personal data.
-   Consistent handling of local dates and time zones.

### 10.6 `context` --- Project overview

Required features:

-   Detect common project types.
-   Summarize directory structure to a configurable depth.
-   Identify version-control status and likely entry points.
-   Identify common build and test configuration files.
-   Exclude sensitive files and file contents by default.

Acceptance criteria:

-   Read-only operation.
-   No automatic upload of project data.
-   Useful partial results for unknown project types.
-   AI analysis, if later added, remains optional and requires clear
    data-sharing behavior.

### 10.7 Recommended extension-first additions

These additions should be treated as a prioritized backlog, not a
promise that every command ships in the first release. The first
independently installed plugin should validate discovery, manifest
parsing, argument forwarding, output forwarding, and exit-code
propagation before more complex plugins are attempted.

#### `dice` --- first end-to-end plugin

Example usage:

-   `forge dice 4d6kh3`
-   `forge dice 2d20`
-   `forge dice coin`
-   `forge pick pizza sushi tacos`

Required behavior:

-   Parse common dice notation: `NdS`, optional keep-highest/keep-lowest
    modifiers such as `4d6kh3`, and documented modifiers only.
-   Reject malformed notation with a concise usage error.
-   Use an operating-system-backed cryptographically secure random
    source where available; do not claim cryptographic suitability for
    all platform backends unless verified.
-   Print individual rolls and the total; support a concise output mode.
-   `pick` must require at least one candidate and preserve candidate
    strings, including spaces when passed as quoted arguments.
-   The plugin must be installable and removable without rebuilding
    Forge CLI.

Exit codes:

-   `0`: successful roll or selection.
-   `2`: invalid notation, missing arguments, or invalid options.
-   `1`: unexpected runtime failure.

Acceptance criteria:

-   A sample executable plugin can be discovered and invoked from a
    clean install.
-   Arguments containing spaces and Unicode are forwarded intact.
-   The engine returns the plugin's exit code unchanged, unless the
    plugin protocol explicitly defines a transport failure.
-   Plugin stdout and stderr are forwarded predictably.

#### `life` --- terminal rendering plugin

Example usage:

-   `forge life`
-   `forge life --width 80 --height 24`
-   `forge life --seed 42 --steps 500`
-   `forge life --headless --steps 100 --output final.txt`

Required behavior:

-   Implement Conway's Game of Life with documented boundary behavior
    (default: finite grid with dead cells beyond the edges).
-   Support a reproducible seed for random initial states.
-   Support terminal-size defaults and explicit dimensions.
-   Detect non-interactive output; require `--headless` or provide a
    clear error rather than emitting uncontrolled terminal escape
    sequences.
-   Restore terminal state on normal exit and handle interruption where
    platform APIs permit.
-   Keep terminal rendering isolated from the core engine.

Exit codes:

-   `0`: simulation completed or user exited normally.
-   `2`: invalid dimensions, seed, or option combination.
-   `1`: terminal initialization or runtime failure.

Acceptance criteria:

-   Repeated runs with the same seed, dimensions, and step count produce
    the same final grid.
-   The plugin runs as an external executable and does not require a
    core rebuild.
-   Tests cover stable patterns, oscillators, and edge behavior.

#### `jump` --- directory frecency and shell-state test

Example usage:

-   `forge jump engine`
-   `forge jump --add work C:\src\work`
-   `forge jump --list`
-   `forge jump --remove work`

Required behavior:

-   Track directory visits and recency/frequency locally.
-   Offer a deterministic exact-name/path match before fuzzy or frecency
    ranking.
-   Exclude nonexistent directories from jump results and provide a
    useful recovery message.
-   Never change the parent shell's directory by printing an unvalidated
    path and asking the shell to evaluate arbitrary plugin output.
-   Use the documented shell-state protocol to request a `cd` action.
    PowerShell and CMD adapters must validate the requested action and
    path.
-   Provide explicit add, list, remove, and clear operations.

Exit codes:

-   `0`: target resolved and shell action emitted, or management
    operation succeeded.
-   `2`: ambiguous/unmatched target or invalid arguments.
-   `1`: storage or unexpected runtime failure.

Acceptance criteria:

-   Directory changes work in PowerShell and CMD through their supported
    adapters.
-   The core remains responsible for applying the validated shell
    action; plugins cannot request arbitrary shell code execution.
-   Usage history survives restarts and can be cleared by the user.

#### `envx` --- named environment profiles

Example usage:

-   `forge envx list`
-   `forge envx use staging`
-   `forge envx show staging`
-   `forge envx set staging API_URL https://staging.example`
-   `forge envx unset staging API_URL`

Required behavior:

-   Store named profiles and variable names in local configuration or a
    dedicated local database.
-   Treat values as sensitive by default in display output: redact
    configured secret-like keys and support explicit reveal.
-   Profile activation must use the shell-state protocol to set or
    remove environment variables in the parent shell.
-   Switching profiles must define what happens to variables set by the
    previous profile. Initial default: remove variables managed by the
    previous active Forge profile, then apply the selected profile;
    never clear unrelated shell variables.
-   Do not persist inherited environment variables automatically.
-   Do not log secret values.

Exit codes:

-   `0`: profile operation succeeded.
-   `2`: unknown profile, invalid variable name, or invalid options.
-   `1`: configuration/storage failure.

Acceptance criteria:

-   Profile changes affect the current shell, not only the Forge child
    process.
-   Tests prove unrelated environment variables remain unchanged.
-   Secret-like values are redacted from normal output and diagnostic
    logs.
-   The command documents that environment values may be visible to
    processes launched from that shell.

#### `killport` --- guarded port-owner termination

Example usage:

-   `forge killport 8000`
-   `forge killport 8000 --yes`
-   `forge killport 8000 --dry-run`

Required behavior:

-   Reuse the port-inspection service instead of duplicating process
    discovery.
-   Display port, protocol, PID, process name, and any available
    executable path before termination.
-   Default to dry-run/preview when requested and require interactive
    confirmation before killing a process.
-   `--yes` may bypass confirmation but must never bypass permission
    errors or target validation.
-   Refuse to terminate Forge CLI itself, critical operating-system
    processes, or ambiguous targets unless a separately documented
    override is implemented.
-   Explain when elevation is required; never silently elevate.

Exit codes:

-   `0`: no process needed termination, or requested termination
    succeeded.
-   `2`: invalid port/options or ambiguous target.
-   `3`: user declined confirmation.
-   `4`: permission denied or process could not be terminated.
-   `1`: unexpected runtime failure.

Acceptance criteria:

-   The process identity is shown before an interactive termination.
-   The default interactive flow requires confirmation.
-   Dry-run never terminates a process.
-   Tests cover no listener, multiple owners, permission failure, and
    user cancellation.

#### Additional backlog candidates

-   `clip`: clipboard history and named snippets; clipboard access
    should be platform-adapted and history retention configurable.
-   `tmpx`: managed scratch directories with TTL cleanup; cleanup must
    never traverse outside the registered scratch root.
-   `rename`: batch renaming with preview, collision detection, and an
    undo journal; no writes until preview is accepted.
-   `qr` and `serve`: QR rendering and local static file serving; LAN
    exposure must be explicit, with a clear bind-address display and
    shutdown instructions.
-   `ago` / `until`: date arithmetic with explicit local-date and
    timezone rules.
-   `gitx`: Git status summary across registered projects.
-   `snip`: searchable code snippets using shared local storage.
-   `jsonx`: JSON formatting, path queries, and structural diff,
    supporting stdin and files.
-   `diffdir`: content-based directory comparison with clear symlink and
    permission behavior.
-   `timeit`: command duration history that can feed benchmark
    comparisons without claiming statistically rigorous benchmarking.
-   `wordle`, `typing`, `radio` / `play`, `rpg`, `matrix`, `pipes`, and
    `pick`: optional recreational plugins that should not increase core
    startup cost.

### 10.8 Recommended implementation order

1.  **External plugin protocol + `dice`:** validate manifests,
    discovery, argument forwarding, output, and exit codes with a
    dependency-light plugin.
2.  **`life`:** validate interactive terminal rendering and cleanup
    without adding core dependencies.
3.  **`jump`:** implement the shell-state protocol with directory
    navigation in PowerShell and CMD.
4.  **`envx`:** extend the same protocol to environment mutations, with
    strict tests preventing unintended variable changes.
5.  **`killport`:** build on `ports` and add confirmation, dry-run, and
    process-safety handling.
6.  **`jsonx`, `clip`, `rename`, `timeit`:** choose next based on daily
    use and implementation cost.

This sequence intentionally separates plugin plumbing from shell-state
behavior and destructive process operations. The most useful daily
commands (`jump`, `envx`, and `killport`) should not be rushed ahead of
their safety and integration tests.

------------------------------------------------------------------------

------------------------------------------------------------------------

## 11. Configuration and Data Storage

### 11.1 Configuration

Use a documented, portable text format such as TOML. Configuration
should support:

-   Defaults.
-   User overrides.
-   Command-specific settings.
-   Environment-variable overrides where appropriate.
-   Validation and clear diagnostics.
-   Safe updates and backup of prior configuration where useful.

Possible settings include:

``` toml
[general]
output = "human"
color = "auto"
timezone = "local"

[plugins]
directory = "plugins"
allow_scripts = true
allow_native_libraries = false

[execution]
default_shell = "powershell"
default_timeout_seconds = 60
```

This is illustrative; exact keys and default values must be finalized
during implementation.

### 11.2 Directory layout

Use OS-appropriate application-data locations rather than requiring the
installation directory to be writable.

Conceptual layout:

``` text
Forge user data/
├── config.toml
├── data/
│   └── forge.db
├── cache/
├── logs/
├── plugins/
├── templates/
└── backups/
```

The executable installation location must remain separate from user data
so upgrades do not destroy notes, tasks, plugins, or configuration.

### 11.3 Persistence

SQLite is recommended for related structured records such as projects,
tasks, notes, reminders, habits, and focus sessions. Do not initialize
the database for commands that do not need it.

Requirements:

-   Schema versioning and migrations.
-   Transactional updates.
-   Appropriate indexes.
-   Integrity checks where useful.
-   Backup and export support.
-   Portable data formats for important user-owned information.

Avoid storing large build artifacts, downloaded files, or unrelated
project content in the database.

------------------------------------------------------------------------

## 12. Security and Safety

### 12.1 Destructive operations

Commands that delete, overwrite, terminate, or modify important
resources must use appropriate safeguards.

-   Provide dry-run or preview modes where practical.
-   Explain the intended changes before execution.
-   Require confirmation for high-impact operations.
-   Do not silently overwrite files.
-   Handle symbolic links and paths outside the intended root
    explicitly.
-   Ensure interrupted operations do not leave persistent data
    inconsistent.

### 12.2 Process execution

-   Prefer process APIs that pass an executable and argument list
    separately.
-   Avoid building shell command strings from untrusted input.
-   Make shell interpretation explicit when it is genuinely required.
-   Apply timeouts where appropriate.
-   Forward signals and clean up child processes predictably.

### 12.3 Secrets and privacy

-   Keep data local by default.
-   Do not require telemetry.
-   Do not dump the full environment or credentials into logs.
-   Avoid storing sensitive command-line arguments in history.
-   Exclude secrets and sensitive files from project summaries by
    default.
-   Make external network access explicit for features that need it.

### 12.4 AI features

AI-assisted features are optional and out of scope for the first core
milestone. They must not automatically execute generated commands.
Proposed commands should be shown for review, risky operations
highlighted, and external data transmission disclosed.

------------------------------------------------------------------------

## 13. Performance and Reliability

### 13.1 Provisional targets

Targets are initial engineering goals to measure on a documented
reference machine.

  ---------------------------------------------------------------------
  Metric                             Initial target
  ---------------------------------- ----------------------------------
  Core startup                       Under 50 ms p95 for a warm-cache,
                                     no-plugin trivial command

  Built-in trivial dispatch          Under 10 ms of measured framework
                                     overhead where feasible

  Plugin discovery                   No full plugin-code loading on
                                     ordinary startup

  Idle background usage              No persistent daemon required by
                                     default

  Configuration access               Load only what the requested
                                     command needs

  Directory traversal                Efficient native or
                                     platform-appropriate APIs where
                                     beneficial

  Network operations                 Configurable timeouts and clear
                                     failure reporting
  ---------------------------------------------------------------------

The benchmark methodology must specify OS version, hardware, build mode,
warm/cold cache conditions, sample count, and whether process creation
is included.

### 13.2 Performance principles

-   Do not scan the entire filesystem during startup.
-   Do not load all plugin executables or libraries.
-   Do not initialize SQLite unnecessarily.
-   Do not perform network requests during command discovery.
-   Cache plugin metadata safely and invalidate it on changes.
-   Filter filesystem candidates early.
-   Allow search roots and excluded directories to be configured.
-   Support cancellation and progress reporting for long operations.
-   Measure external process startup separately from engine dispatch.

### 13.3 Reliability

-   Failures must produce useful diagnostics rather than unexplained
    crashes.
-   Optional extension failure must not prevent unrelated commands from
    working.
-   Persistent data must survive ordinary command failures.
-   Tests must cover argument parsing, validation, persistence,
    shell-state actions, and extension discovery.
-   Long-running operations should handle interruption and cleanup where
    practical.

------------------------------------------------------------------------

## 14. Testing Strategy

### 14.1 Unit tests

Cover argument parsing, command registration, configuration validation,
output formatting, error mapping, path normalization, and storage logic.

### 14.2 Integration tests

Cover dispatcher-to-command flow, plugin manifests, child-process
argument forwarding, output and exit-code handling, SQLite persistence,
and shell integration.

### 14.3 Security and safety tests

Verify that:

-   Invalid manifests are rejected.
-   Command collisions are detected.
-   Dry-run operations do not modify resources.
-   Existing files are not silently overwritten.
-   Plugin paths cannot escape the validated installation directory
    through unintended path traversal.
-   Shell adapters do not execute arbitrary text returned by a plugin.
-   Sensitive information is not included in routine logs.
-   Interrupted operations leave consistent data.

### 14.4 Performance tests

Track startup latency, built-in dispatch overhead, plugin discovery
time, memory use, and representative filesystem workloads. Keep
benchmark results reproducible and avoid treating noisy measurements as
regressions without repeated evidence.

### 14.5 Platform tests

For Windows, test PowerShell and CMD separately. In future platform
ports, test shell-state behavior, path handling, permissions, process
management, and terminal output independently.

------------------------------------------------------------------------

## 15. Proposed Repository Layout

The final layout should follow the chosen language's conventions. This
conceptual structure shows the intended separation:

``` text
forge/
├── README.md
├── LICENSE
├── docs/
│   ├── architecture.md
│   ├── commands.md
│   ├── plugin-development.md
│   ├── shell-integration.md
│   └── security.md
├── core/
│   ├── dispatcher/
│   ├── registry/
│   ├── config/
│   ├── plugin_loader/
│   ├── process/
│   ├── output/
│   └── logging/
├── commands/
│   ├── help/
│   ├── version/
│   ├── config/
│   └── plugin/
├── sdk/
│   └── plugin_api/
├── integrations/
│   ├── powershell/
│   ├── cmd/
│   └── future/
├── templates/
├── tests/
│   ├── unit/
│   ├── integration/
│   ├── plugin/
│   └── performance/
└── packaging/
    ├── windows/
    └── future/
```

For Rust, use a Cargo workspace and crates as appropriate. For C++, use
a suitable CMake structure. The directory names are not a requirement to
create one project or build target per folder.

------------------------------------------------------------------------

## 16. Implementation Roadmap

### Phase 0 --- Architecture decisions

Finalize the core language, plugin manifest schema, command API,
shell-state protocol, configuration format, installation model, and
target Windows versions.

**Exit criterion:** Interfaces and security boundaries are documented
before implementation begins.

### Phase 1 --- Minimal engine

Implement argument parsing, dispatch, `help`, `version`, consistent
errors, exit codes, and basic tests.

**Exit criterion:** A new sample command can be added without modifying
the dispatcher.

### Phase 2 --- PowerShell and CMD integration

Implement adapters for ordinary command execution and validated
shell-state actions.

**Exit criterion:** Commands run in the current terminal, and project
navigation changes the active shell's directory correctly in both
supported shells.

### Phase 3 --- External executable plugins

Implement manifests, plugin discovery, validation, command registration,
argument forwarding, output forwarding, exit-code handling, and local
installation/removal. Use `dice` as the first end-to-end sample plugin,
then use `life` to validate terminal interaction outside the core.

**Exit criterion:** Separately built `dice` and `life` plugins can be
installed and invoked without rebuilding the engine; argument
forwarding, output, and exit-code behavior are covered by automated
tests.

### Phase 4 --- Shared host services and core utilities

Implement documented configuration, logging, output, filesystem helpers,
plus initial commands such as `proj`, `mkproj`, `sysinfo`, and `ports`.

**Exit criterion:** Initial utilities use stable shared interfaces and
have automated tests.

### Phase 5 --- Productivity and persistence

Add SQLite-backed tasks, notes, and the daily dashboard where
appropriate.

**Exit criterion:** Data survives restarts and can be exported.

### Phase 6 --- Advanced developer utilities

Add benchmarking, project context, file watching, workflows, backup, and
restore.

**Exit criterion:** Advanced features work without bloating the core
startup path.

### Phase 7 --- Native plugin ABI

Implement versioned C-compatible shared-library loading, compatibility
checks, ownership rules, and failure handling.

**Exit criterion:** A separately built native plugin can register and
execute commands without recompiling the engine.

### Phase 8 --- Packaging and distribution

Provide installation, upgrade, configuration preservation, uninstall
behavior, documentation, and optional plugin integrity checks.

**Exit criterion:** A user can install, update, and uninstall the
utility without losing personal data.

### Phase 9 --- Additional platforms

Add Linux/macOS support and appropriate shell adapters after the Windows
interfaces are stable.

**Exit criterion:** Shared behavior passes platform-specific integration
tests without leaking Windows-only assumptions into the core.

------------------------------------------------------------------------

## 17. Documentation Requirements

The repository should include:

-   Installation and upgrade guide.
-   Quick-start guide.
-   Command reference.
-   Configuration reference.
-   Plugin installation and security guide.
-   Plugin development guide.
-   Shell integration guide.
-   Troubleshooting guide.
-   Contribution and testing instructions.
-   Performance benchmarking methodology.

Each command should have copyable examples and clearly documented output
and exit-code behavior.

------------------------------------------------------------------------

## 18. Initial Release Acceptance Criteria

The initial release is considered usable when:

1.  The compiled executable starts and dispatches registered commands.
2.  Global and command-specific help are available.
3.  Invalid arguments produce clear errors.
4.  Configuration persists across runs.
5.  A new external executable plugin can be installed without rebuilding
    the engine.
6.  Plugin manifests are validated and command conflicts are detected.
7.  The PowerShell and CMD adapters execute ordinary commands correctly.
8.  A shell-state command can change the current directory through each
    adapter.
9.  Detached and new-shell modes have documented, testable behavior.
10. Project records can be added, listed, searched, and opened.
11. Template generation does not silently overwrite existing files.
12. Port inspection handles unavailable permissions gracefully.
13. Automated tests cover core dispatch and plugin execution.
14. Destructive operations include appropriate safeguards.
15. User data survives normal upgrades and can be backed up or exported.
16. Startup and dispatch performance are measured against documented
    targets.

The initial release does not need every command in the catalogue or
native shared-library plugins. Those are later milestones.

------------------------------------------------------------------------

## 19. Open Decisions

The following items must be finalized before implementation:

  ---------------------------------------------------------------------
  Decision                           Current recommendation
  ---------------------------------- ----------------------------------
  Core language                      Rust; keep C++ as an acceptable
                                     alternative

  Plugin manifest                    TOML

  First plugin mechanism             External executables

  Native plugin interface            Versioned C ABI in a later phase

  Script extensions                  Optional and explicitly enabled

  Plugin source                      Local installation initially;
                                     repository later

  Shell integration                  PowerShell module and CMD adapter

  Shell-state protocol               Versioned structured response with
                                     allowlisted actions

  Database                           SQLite only for features requiring
                                     structured persistence

  Interactive TUI                    Optional later feature

  AI integration                     Out of scope for the initial core

  Background daemon                  Not required by default

  Distribution                       Windows installer/package approach
                                     to be selected during
                                     implementation
  ---------------------------------------------------------------------

------------------------------------------------------------------------

## 20. Definition of Success

Forge CLI succeeds when it becomes a dependable part of daily terminal
use: common operations become faster, project and personal utilities are
easy to access, and new capabilities can be installed without rebuilding
the core.

The engine should remain small and maintainable as the command ecosystem
grows. Plugins should be independently developed, documented, validated,
and removable. Shell integration should feel natural without pretending
that a child process can directly mutate its parent shell.

The long-term vision is a personal, native command-line environment that
can be extended like a mod-friendly application while retaining the
performance, predictability, and safety expected from a systems utility.
