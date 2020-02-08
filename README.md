# static-rs
Bart Massey

This demos compilation and linking of code against external
C/UNIX stuff three different ways:

* By printing to UNIX standard output directly, bypassing
  Rust `stdout()`.

* By printing to `libc`/UNIX standard output using
  `libc::write()`.

* By calling an external C function to print to standard
  output.

There are two intended binary targets:

* Statically-linked against the Rust stuff but dynamically
  against the C/UNIX stuff: `cargo build`

* Fully statically-linked:

     * Make sure a statically-linkable toolchain
     `x86_64-unknown-linux-musl` or the like is installed.

     * `cargo build --target x86_64-unknown-linux-musl`

## License

This program is licensed under the "MIT License". Please see
the file `LICENSE` in this distribution for license terms.
