# Pathcomp

**Pathcomp** is a minimal tool for compressing a `$PATH`-like environment
variables so that:

- Individual paths occur only once
- The (effective) ordering of paths remains unchanged

## Installation

```.sh
$ git clone
$ cd pathcomp
$ cargo install
```

## Usage

Typically you would have something like in your `.profile`:

```.sh
export PATH=$(pathcomp "$PATH")
```

In case you are unsure if `pathcomp` is available:

```.sh
if which pathcomp
then
    PATH=$(pathcomp "$PATH")
fi
export PATH
```
