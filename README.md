# Pathcomp

**Pathcomp** is a minimal tool for compressing a `$PATH`-like environment
variables so that:

- Individual paths occur only once
- The (effective) ordering of paths remains unchanged

## Installation

```.sh
$ git clone git@github.com:mjhanninen/pathcomp.git
$ cd pathcomp
$ cargo install --path .
```

## Usage

Typically you would have something like in your `.profile`:

```.sh
export PATH=$(pathcomp "$PATH")
```

In case you are unsure if `pathcomp` is available:

```.sh
if which pathcomp &>/dev/null; then
    PATH=$(pathcomp "$PATH")
fi
export PATH
```

## How it works?

### Deduplication

**Pathcomp** deduplicates paths while retaining the order of the first path
instances:

```console
$ echo $(pathcomp "/foo:/bar:/foo:/bar")
/foo:/bar
```

```console
$ echo $(pathcomp "/bar:/foo:/bar:/foo")
/bar:/foo
```

### Reordering

**Pathcomp** pulls the paths matching the prefixes specified
with the `--prefix` (or `-p`) options to the front.  If case multiple prefixes
match the same path then the prefix with longest match is used:

```console
$ echo $(pathcomp -p /foo "/foo:/bar:/foo/bar/baz:/foo/bar")
/foo:/foo/bar/baz:/foo/bar:/bar
```

```console
$ echo $(pathcomp -p /foo/bar -p /foo "/foo:/bar:/foo/bar/baz:/foo/bar")
/foo/bar/baz:/foo/bar:/foo:/bar
```
