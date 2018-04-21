# Sac

A set of tools to work with Registers.

* * *

**Nothing useful here**

* * *



## CLI Design

### Validate item

What makes this command useful?

* Describe all issues or just the first one. The first one is much simpler to
  implement given how serde_json works.
* …

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

### Canonicalise item

```sh
$ sac item canon '{"c": "z", "a": "x", "b": "y"}'
{"a":"x","b":"y","c":"z"}

$ echo '{"a": "x", "b": "y", "c": "z"}' | sac item canon
{"a":"x","b":"y","c":"z"}

$ echo $?
0

$ sac item canon '{"A": "z", "a": "x", "b": "y"}'
invalid field name "A"

$ echo $?
1

$ sac item canon --force '{"A": "z", "a": "x", "b": "y"}'
{"a":"x","b":"y"}

$ echo $?
0
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

### Hash item

```sh
$ sac item hash '{"a":"x","b":"y","c","z"}'
ecd26bd54edf231ecbfbe361c97e0f720068f562e26c32696e777b6ed494cf73

$ sac item hash '{"a":"x", "b":"y","c","z"}'
invalid item. it should be canonical

$ sac item hash --force '{"a":"x", "b":"y","c","z"}'
ecd26bd54edf231ecbfbe361c97e0f720068f562e26c32696e777b6ed494cf73

$ sac item hash --with-alg '{"a":"x","b":"y","c","z"}'
sha-256:ecd26bd54edf231ecbfbe361c97e0f720068f562e26c32696e777b6ed494cf73
```

### Mint item


```
$ sac item add --force '{"a":"x", "b":"y","c","z"}'
add-item	{"a":"x","b":"y","c","z"}

$ sac entry append --pk a 'add-item	{"a":"x", "b":"y","c","z"}'
append-entry	a	2018-04-13T15:12:00Z	sha-256:ecd26bd54edf231ecbfbe361c97e0f720068f562e26c32696e777b6ed494cf73
```

Or

```
$ sac mint --force --pk a '{"a":"x", "b":"y","c","z"}'
add-item	{"a":"x","b":"y","c","z"}
append-entry	a	2018-04-13T15:12:00Z	sha-256:ecd26bd54edf231ecbfbe361c97e0f720068f562e26c32696e777b6ed494cf73
```

## Development

Lint with Clippy. E.g.

```sh
cargo +nightly install clippy
cargo +nightly clippy
```
