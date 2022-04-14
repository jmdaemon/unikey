# Unikey

A custom keyboard layout creation tool for Linux and MacOSX systems.

Unikey is a tool to help you create and install custom keyboard layouts on Linux.

# Building

To build unikey from source you can run `cargo build` to build unikey with cargo.

## Installation

To install unikey you can run `cargo install unikey` to install unikey to your system.

Alternatively you can also install unikey through your system's package manager by installing
one of the available releases.

## Usage

### Creating your keyboard layout

1. Create a keyboard layout:
``` toml
[config]
name = "us"
desc = "English (US)"

[rows]
    e = [ '1', '2', '3', '4', '5', '6', '7', '8'    , '9'       , '0'           , 'minus'       , 'equal' ]
    d = [ 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i'    , 'o'       , 'p'           , 'bracketleft' , 'bracketright' ]
    c = [ 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k'    , 'l'       , 'semicolon'   , 'apostrophe' ]
    b = [ 'z', 'x', 'c', 'v', 'b', 'n', 'm', 'comma', 'period'  , 'slash' ]

    [rows.misc]
    BKSL = "backslash"
    TLDE = "grave"
```

2. Compile your keyboard layout for the target operating system:

``` bash
unikey keyboard.layout.toml layout -t linux
```

3. Install using the `install.sh` script in the project:

```
./install.sh layout layout_name
```

You can also do a dryrun by passing in the `-d` flag when running unikey.

For more information about creating your keyboard layout,
see `key.layout.toml` for an example of a keyboard layout.

Note that for linux keyboard maps, your key values are not checked for validity at creation time,
so if you install the keyboard map containing an invalid or unrecognized key value, you will be
unable to load your custom keyboard layout.

## Contributions

Contributions are welcomed. If you need to contact me you can do so via my [email](mailto:josephm.diza@gmail.com).
