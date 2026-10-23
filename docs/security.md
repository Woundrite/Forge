# Security model

Forge is local-first and does not perform network requests during ordinary
command discovery. Plugins execute with the current user's permissions and
must be treated as trusted code.

The core protects important boundaries by:

- passing child arguments without concatenating shell command strings;
- rejecting plugin executable paths that escape their plugin directory;
- rejecting invalid manifests and command collisions;
- separating machine-readable output from diagnostics;
- defining a small, versioned shell-state protocol;
- avoiding automatic environment or secret logging.

Destructive commands must provide preview or confirmation behavior where
appropriate. Security-sensitive changes should be reviewed through a merge
request and disclosed privately rather than in a public issue.
