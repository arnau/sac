# Name

sac - A set of tools to work with Registers

# Synopsis

**sac** [*SUBCOMMAND*]

**sac** item [*SUBCOMMAND*]

# Description

**sac** is a set of tools to work with Registers from porcelain down to plumbing.

You should always expect an exit code of `0` when the command succeeds.
Failure codes are being designed so in the meantime all of them are `1`.

## Types

The following data types can be used to represent values:

* **bool** — Boolean (e.g. `true`, `false`).
* **curie** — Curie (e.g. `example:foo`).
* **datetime** — UTC ISO8601 date time in any multiple accuracies (e.g. `2018`, `2016-10-11T12:13:14Z`).
* **hash** — Qualified hash (with algorithm) (e.g. `sha-256:ecd26bd54edf231ecbfbe361c97e0f720068f562e26c32696e777b6ed494cf73`).
* **inapplicable** — Inapplicable value (e.g. `N/A`).
* **integer** — Signed integer (e.g. `3`, `-10`).
* **period** — ISO8601 period in any multiple forms and accuracies (e.g. `P1Y2M`, `PT10H`).
* **point** — WKT point as defined by OGC 06-104r4 (OpenGIS® Implementation Standard for Geographic information - Simple feature access - Part 2: SQL Option) (e.g. `POINT (10 25)`).
* **polygon** — WKT polygon as defined by OGC 06-104r4 (OpenGIS® Implementation Standard for Geographic information - Simple feature access - Part 2: SQL Option) (e.g. `POLYGONZ ((0 0 1, 1 1 1, 2 2 1))`).
* **string** — UTF-8 string.
* **text** — Common Markdown text.
* **timestamp** — RFC3339 UTC timestamp (e.g. `2018-06-07T08:09:10Z`).
* **url** — Url (e.g. `https://example.org/foo`).

## Blobs

* `blob canon` - Takes a blob and transforms it into its canonical form.
* `blob hash` - Takes an blob and generates its hash.

## Values

* `value check --type <type>` - Checks a value againt a type.

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
