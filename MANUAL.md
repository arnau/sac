# Name

sac - A set of tools to work with Registers

# Synopsis

**sac** [*SUBCOMMAND*]

**sac** item [*SUBCOMMAND*]

# Description

**sac** is a set of tools to work with Registers from porcelain down to plumbing.

You should always expect an exit code of `0` when the command succeeds.
Failure codes are being designed so in the meantime all of them are `1`.

## Items

* `item canon` - Takes an item and transforms it into its canonical form.
* `item hash` - Takes an item and generates its hash.

# Examples

Canonicalise an item:

```sh
$ sac item canon '{"foo": "abc", "bar": "xyz"}'
{"bar":"xyz","foo":"abc"}
```

Hash an item:

```sh
$ sac item hash '{"bar":"xyz","foo":"abc"}'
5dd4fe3b0de91882dae86b223ca531b5c8f2335d9ee3fd0ab18dfdc2871d0c61
```

# Author

Copyright 2018 Arnau Siches (asiches@gmail.com). This software carries no
warranty of any kind.
