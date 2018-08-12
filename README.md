# Sac

A System of Record toolkit.

* * *

**Nothing useful here**

* * *

## Install

```sh
cargo build --release
cp target/release/sac $MYBINS/sac
```

## Usage

See the [manual](MANUAL.md)


### Blobs

`sac blob` offers a set of tools to work with blobs (also known as items). For
example, you can canonicalise a blob `sac blob canon` or compute its hash `sac
blob hash`.

* [x] `sac blob canon` — Canonicalise item (json).
* [ ] `sac blob canon --from csv` — Canonicalise item (csv).
* [x] `sac blob hash` — Hash item (SHA-2 256).
* [ ] `sac blob hash -a blake2` — Hash item (non SHA-2 256).

### Values

`sac value` offers a set of tools to work with values. For example, you can
check if a value conforms with a given datatype:

```sh
sac value check --type <type>
```

### types

* [x] `bool` — Boolean (true / false).
* [x] `curie` — Curie.
* [x] `datetime` — ISO8601 date time in any multiple accuracies.
* [x] `hash` — Qualified hash (with algorithm).
* [x] `inapplicable` — Inapplicable value (N/A).
* [x] `integer` — Signed integer.
* [x] `period` — ISO8601 period in any multiple forms and accuracies.
* [x] `point` — WKT point as defined by OGC 06-104r4 (OpenGIS® Implementation Standard for Geographic information - Simple feature access - Part 2: SQL Option).
* [x] `polygon` — WKT polygon as defined by OGC 06-104r4 (OpenGIS® Implementation Standard for Geographic information - Simple feature access - Part 2: SQL Option).
* [x] `string` — UTF-8 string.
* [x] `text` — Common Markdown text.
* [x] `timestamp` — RFC3339 UTC timestamp.
* [x] `url` — Url.


## CLI Design

Ideas to be implemented.

### Validate item

What makes this command useful?

```sh
$ sac item validate '{"c": "z", "a": "x", "b": "y"}'
{"issues": [["not-canonical"]]}

$ echo $?
1

$ sac item validate '{"a":"x","b":"y"}'
{"issues": []}

$ echo $?
0

$ sac item validate '{"A":"x","b":"y"}'
{"issues": ["invalid-fieldname"]}
```


Consider accepting CSV or TSV as inputs if annotated with headers.

```sh
$ sac item canon --format=csv --header 'a,b,c' 'x, y,c'
{"a":"x","b":"y","c":"z"}
```

What happens with CSV with multiple rows? MAYBE it should be a different
command.

```sh
$ cat foo.csv
a,b,c
x1,y1,z1
x2,y2,z2

$ sac item canon --format csv --input foo.csv
{"a":"x1","b":"y1","c","z1"}
{"a":"x2","b":"y2","c","z2"}
```


### Mint item


```
$ sac mint --force --pk a '{"a":"x", "b":"y","c","z"}'
add-item	{"a":"x","b":"y","c","z"}
append-entry	a	2018-04-13T15:12:00Z	sha-256:ecd26bd54edf231ecbfbe361c97e0f720068f562e26c32696e777b6ed494cf73
```

## Development

Lint with Clippy:

```sh
cargo +nightly install clippy
cargo +nightly clippy
```


Format with rustfmt:

```sh
rustup component add rustfmt-preview
cargo fmt
```
