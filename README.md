# Unikey

A custom keyboard creation tool for Linux and MacOSX systems.

Unikey is a tool to help you create and install custom keyboard layouts on Linux.

# Building

To build unikey from source you can run `cargo build` to build unikey with cargo.

## Installation

To install unikey you can run `cargo install unikey` to install unikey to your system.

Alternatively you can also install unikey through your system's package manager by installing
one of the available releases.

## Usage

To create your keyboard layout you can run:

``` bash
unikey keyboard.layout.toml layout -t linux
```

to generate a linux xkb compatible keyboard mapping for you to use.
You can also do a dryrun by passing in the `-d` flag when running unikey.

### Installing your custom keyboard mappings

To install your custom keyboard map, use the `install.sh` file in the repository:

```
./install.sh layout layout_name
```

you will be prompted to enter the root password to install the keyboard map.

## Contributions

Contributions are welcomed. If you need to contact me you can do so via my [email](mailto:josephm.diza@gmail.com).
