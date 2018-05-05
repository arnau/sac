# Name

sac - A set of tools to work with Registers

# Synopsis

**sac** [*SUBCOMMAND*]

**sac** item [*SUBCOMMAND*]

# Description

**sac** is a set of tools to work with Registers from porcelain down to plumbing.

You should always expect an exit code of `0` when the command succeeds.
Failure codes are being designed so in the meantime all of them are `1`.

## Blobs

* `blob canon` - Takes a blob and transforms it into its canonical form.
* `blob hash` - Takes an blob and generates its hash.

## Values

* `value check` - Checks a value againt a type.

# Examples

Canonicalise a blob:

```sh
$ sac blob canon '{"foo": "abc", "bar": "xyz"}'
{"bar":"xyz","foo":"abc"}
```

Hash a blob:

```sh
$ sac blob hash '{"bar":"xyz","foo":"abc"}'
5dd4fe3b0de91882dae86b223ca531b5c8f2335d9ee3fd0ab18dfdc2871d0c61
```

Check a URL:

```sh
$ sac value check --type url https://example.org/
The value https://example.org/ is a valid Url
```

# Author

Copyright 2018 Arnau Siches (asiches@gmail.com). This software carries no
warranty of any kind.
