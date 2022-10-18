# linurgy

[![Crates.io](https://img.shields.io/crates/v/linurgy.svg)](https://crates.io/crates/linurgy)
[![msrv 1.32](https://img.shields.io/badge/msrv-1.32-dea584.svg)](https://github.com/rust-lang/rust/releases/tag/1.32.0)
[![Documentation](https://docs.rs/linurgy/badge.svg)](https://docs.rs/linurgy/)
[![Rust CI](https://github.com/sonro/linurgy/actions/workflows/rust.yml/badge.svg)](https://github.com/sonro/linurgy/actions/workflows/rust.yml)
[![license](https://img.shields.io/crates/l/linurgy.svg)](#license)

Rust library to manipulate multiple newlines.

Create a new `String` with your edited text, or use buffers to pipe input and output into the
`Editor`. This library has no additional dependencies.

## Using linurgy

Build a reusable `Editor` with one of the convenient `factory` functions. Use the `edit` method to
create a new `String`.

```rust
use linurgy::factory;

// appends an underscore "_" every 2 newlines "\n\n" => "\n\n_"
let editor = factory::appender("_", 2);
let output = editor.edit("foo\n\n");
assert_eq!("foo\n\n_", output);
```

Manipulate `stdin` into `stdout` by using the `edit_buffered` method. This also works on files,
`Cursor`s, or anything else that implements
[`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html).

```rust
use linurgy::factory;
use std::io::{BufReader, Result, stdin, stdout};

// doubles every newline "\n" => "\n\n"
let editor = factory::appender("\n", 1);
// create a buffer over stdin
let mut input = BufReader::new(stdin());
// pipe input into editor and output to stdout
editor.edit_buffered(&mut input, &mut stdout())?;
```

Work with LF `\n` or CRLF `\r\n` line-endings. There are `factory` functions for CRLF inputs.

```rust
use linurgy::factory;

// inserts a "*" before 2 newlines "\r\n\r\n" => "*\r\n\r\n"
let editor = factory::inserter_crlf("*", 2);
let output = editor.edit("foo\r\nbar\r\n\r\n");
// notice there is only an asterisk before the double newline
assert_eq!("foo\r\nbar*\r\n\r\n", output);
```

[More examples](examples/)

## License

Linurgy is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
