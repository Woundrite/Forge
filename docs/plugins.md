# Plugin catalog

The current plugin catalog intentionally starts with simple, easy-to-test
examples that validate the Forge extension contract end-to-end.

## Sample plugins

### `hello`

Usage:

```powershell
forge hello
forge hello Ada Lovelace
```

Returns a friendly greeting and exits successfully.

### `pick`

Usage:

```powershell
forge pick pizza sushi tacos
```

Chooses one candidate from the supplied list and exits with code `0`.

### `dice`

Usage:

```powershell
forge dice 2d20
forge dice 4d6kh3
```

Parses common dice notation, prints each roll and the sum, and exits with
`0` on success or `2` for invalid notation.

## Next plugins to implement

The next logical candidates from the specification are the shell-state helpers:

- `jump`: directory frecency + `cd` shell-state actions.
- `envx`: named environment profiles.
- `workflows` or `daily`: task and note tracking flows.

These will follow the same external-plugin pattern, but they will also rely on
`ShellResponse` validation and the shell adapter security model.
