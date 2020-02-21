# linurgy

[![Crates.io](https://img.shields.io/crates/v/linurgy.svg)](https://crates.io/crates/linurgy)
[![Documentation](https://docs.rs/linurgy/badge.svg)](https://docs.rs/linurgy/)
[![Build Status](https://travis-ci.org/sonro/linurgy.svg?branch=master)](https://travis-ci.org/sonro/linurgy)

Rust library to manipulate multiple newlines. Works with stdin and stdout, files, and buffers. No dependencies.

Linurgy provides an interface for manipulating multiple newlines in text.
Interaction with this library happens through `LinurgyBuilder`.

## Examples

Read stdin and for each empty line, append an extra line to stdout.

```rust
use linurgy::LinurgyBuilder;

LinurgyBuilder::new()
    .set_newline_trigger(2)
    .set_new_text(String::from("\n"))
    .run()?;
```

Read from one buffer, remove all empty lines, and output to another buffer.

```rust
use linurgy::{LinurgyBuilder, Input, Output, EditType};
let input = String::from("Remove\n\nEvery\n\nEmpty\n\nLine\n");
let mut output = String::new();

LinurgyBuilder::new()
    .set_input(Input::Buffer(&input))
    .set_output(Output::Buffer(&mut output))
    .set_newline_trigger(2)
    .set_edit_type(EditType::Replace)
    .set_new_text("\n")
    .run()?;

assert_eq!("Remove\nEvery\nEmpty\nLine\n", &output);
```
