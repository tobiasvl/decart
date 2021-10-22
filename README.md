decart
======
[![crates.io](https://img.shields.io/crates/v/decart.svg)](https://crates.io/crates/decart)
[![docs.rs](https://img.shields.io/docsrs/decart.svg)](https://docs.rs/decart)
[![dependency status](https://deps.rs/crate/decart/0.0.3/status.svg)](https://deps.rs/crate/decart/)

`decart` is a library and (in the future) a command-line tool for reading and generating "Octocarts",
CHIP-8 game cartridges for the [Octo](https://github.com/JohnEarnest/Octo) environment, written in Rust.

Use cases
---------

* Decoding: You can extract the program source code and runtime settings from an
  Octocart file. The source code can be assembled into CHIP-8 bytecode with Octo or
  [`decasm`]. The runtime settings can be given to a CHIP-8 interpreter like Octo or
  [`deca`], or saved as JSON for the [CHIP-8
  Archive](https://github.com/JohnEarnest/chip8Archive),
or (in the future) as an `.octo.rc` file for [C-Octo](https://github.com/JohnEarnest/c-octo) or [`termin-8`](https://crates.io/crates/termin-8), etc.
* Encoding: TODO

Octocarts
---------

Octo cartridge files, or Octocarts, are GIF89a images with a payload steganographically
embedded in one or more animation frames. Data is stored in the least significant
bits of colors, 1 from the red/blue channels and 2 from the green channel,
allowing us to pack a hidden byte into every 2 successive pixels.

The payload consists of a 32-bit length, followed by a sequence of ASCII bytes
consisting of the JSON-encoded options dictionary and source text.

An Octo cartridge contains the source code of an Octo program, and a set of
configuration options (parsed by [`octopt`](https://crates.io/crates/octopt)
for the CHIP-8 interpreter telling it how to run the program.

See also
--------

* To compile/assemble the Octo source code in the Octocart, check out the [`decasm`](https://crates.io/crates/decasm) crate.
* To interpret an assembled program, check out the [`deca`](https://crates.io/crates/deca) crate (backend) or
  a program like [`termin-8`](https://crates.io/crates/termin-8) (frontend and graphics).
