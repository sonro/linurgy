/*!
An easy to use library for manipulating multiple newlines.

Create an [`Editor`] with one of the six [`factory`] functions to suit your line editing needs.
Or create one directly with [`Editor::new`].

# Examples

## General

Replace all newlines with spaces.

```rust
# use linurgy::factory;
let editor = factory::replacer(" ", 1);
let output = editor.edit("foo\nbar");
assert_eq!("foo bar", output);
```

Add a tab *after* every double newline.

```rust
# use linurgy::factory;
let editor = factory::appender("\t", 2);
let output = editor.edit("foo \n bar \n\n baz");
assert_eq!("foo \n bar \n\n\t baz", output);
```

## Working with CRLF

Insert dahes *before* every CRLF newline.

```rust
# use linurgy::factory;
let editor = factory::inserter_crlf("---", 1);
let output = editor.edit("foo\r\nbar");
assert_eq!("foo---\r\nbar", output);
```

Replace CRLF with a LF.

```rust
# use linurgy::factory;
let editor = factory::replacer_crlf("\n", 1);
let output = editor.edit("foo\r\nbar");
assert_eq!("foo\nbar", output);
```

## Working with buffered streams

Add double lines from [`stdin`](std::io::stdin) to [`stdout`](std::io::stdout).

```rust
# use linurgy::factory;
# use std::io::{BufReader, Result, stdin, stdout};
#
# fn main() -> Result<()> {
let editor = factory::appender("\n", 1);
let mut input = BufReader::new(stdin());
editor.edit_buffered(&mut input, &mut stdout())?;
#
# Ok(())
# }
```

Add an extra line of dashes to every 2 newlines. Using a [`Cursor`](std::io::Cursor) as input (type
that implments [`BufReader`](std::io::BufReader)). Output is a [`Vec<u8>`] (type that implements
[`Write`](std::io::Write)).

```rust
# use std::error::Error;
# use std::io::Cursor;
# use std::str::from_utf8;
# use linurgy::factory;
#
# fn main() -> Result<(), Box<dyn Error>> {
let editor = factory::appender("---\n", 2);
let mut input = Cursor::new("foo\n\nbar");
let mut output = Vec::new();
editor.edit_buffered(&mut input, &mut output)?;
assert_eq!("foo\n\n---\nbar", from_utf8(&output)?);
#
# Ok(())
# }
```
*/

mod editor;
pub mod factory;
mod legacy;

pub use editor::{Editor, NewlineType};
pub use factory::EditType;
pub use legacy::*;
