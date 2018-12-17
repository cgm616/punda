punda: programming the micro:bit with Rust
=====

[![Travis](https://img.shields.io/travis/cgm616/punda/master.svg)](https://travis-ci.org/cgm616/punda)
[![License](https://img.shields.io/badge/license-mit-blue.svg)](https://github.com/cgm616/punda/blob/master/LICENSE)

Punda provides a high level, easy to use interface for interacting with the [BBC micro:bit](http://microbit.org/), a small, handheld micro-controller designed for teaching and learning computer science, using the [Rust Programming Language](https://www.rust-lang.org/).

The intention of `punda` is to complement the other development environments already available for the micro:bit: the Javascript Blocks editor and MicroPython.
Specifically, `punda` brings Rust's static typing, ergonomics, and functional constructs to the table.
In addition to being useful for teaching programming to those unfamiliar, `punda` is also a good way to learn Rust: two equivalent micro:bit programs can be compared across Javascript, Blocks, Python, and Rust to highlight the similarities and differences between each.

For examples of using the `punda` crate, check the `examples/` directory in this repository.
With the correct development environment set up, any example can be run on the micro:bit with `cargo run --example [name]`.

## Under construction
Note that this crate's documentation, API, and entire design is still up for determination.
As a hobby project, I don't have as much time to work on it as I wish I could.

## Development dependencies

To develop applications using this library, the following tools must be installed:

- rustup, Rust's toolchain manager
- gdb-arm-none-eabi, a build of gdb compatible with the micro:bit
- OpenOCD, a program to communicate with and flash the micro:bit

When the above tools are installed, perform the following setup to start a new binary crate (application) using `easy_microbit`.

First, install the nightly Rust toolchain.

```
> rustup install nightly-2018-09-27
```

Next, install the `thumbv6m-none-eabi` Rust target.

```
> rustup target add thumbv6m-none-eabi
```

Create a new Rust crate.

```
> cargo new --bin microbit_example
```



