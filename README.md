easy_microbit
========

This is a crate that provides a high level, easy to use interface for interacting with the [BBC micro:bit](http://microbit.org/), a small, handheld micro-controller designed for teaching and learning computer science.

The intention of `easy_microbit` is to complement the other development environments already available for the micro:bit: the Javascript Blocks editor and MicroPython.
Specifically, `easy_microbit` brings the Rust Programming Language's static typing, ergonomics, and functional constructs to the table.
In addition to being useful for teaching programming to those unfamiliar, `easy_microbit` is also a good way to learn Rust: two equivalent micro:bit programs can be compared across Javascript, Blocks, Python, and Rust to highlight the similarities and differences between each.

For examples of using the `easy_microbit` crate, check the `examples/` directory in this repository.
With the correct development environment set up, any example can be run on the micro:bit with `cargo run --example [name]`.

# Development dependencies

To develop applications using this library, the following tools must be installed:

- rustup, Rust's toolchain manager
- gdb-arm-none-eabi, a build of gdb compatible with the micro:bit
- OpenOCD, a program to communicate with and flash the micro:bit

When the above tools are installed, perform the following setup to start a new binary crate (application) using `easy_microbit`.

First, install the nightly Rust toolchain.

```
> rustup install nightly-2018-08-06
```

Next, install the `thumbv6m-none-eabi` Rust target.

```
> rustup target add thumbv6m-none-eabi
```

Create a new Rust crate.

```
> cargo new --bin microbit_example
```



