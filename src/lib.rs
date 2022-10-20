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
that implments [`BufRead`](std::io::BufRead)). Output is a [`Vec<u8>`] (type that implements
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

/// Line-ending text editor
///
/// This is a text editor that replaces line-endings with a specified string.
/// This replacement only happens when the number of newlines matches the
/// specified trigger. For example, if the trigger is 2, then the editor
/// replaces two newlines (`\n\n`) with the replacement string.
///
/// The editor can be reused.
///
/// # Basic and buffered
///
/// Use the [`Editor::edit`] method to edit an input [`&str`]. This returns a
/// [`String`] containing the edited text.
///
/// Use the [`Editor::edit_buffered`] method to edit a
/// [`BufRead`](std::io::BufRead) into a [`Write`] output. This is useful
/// for editing files, stdio, or other streams.
///
/// # Newline type
///
/// When constructing an editor, you need to specify the type of newline to use.
/// This can be either [`NewlineType::Lf`] (`\n`) or [`NewlineType::Crlf`]
/// (`\r\n`).
///
/// # Factory
///
/// Users of this library are encouraged to use the [`factory`](crate::factory)
/// functions. These provide convient ways to create instances of this type.
///
/// For example, to append dashes to each newline:
///
/// ```rust
/// # use linurgy::{Editor, NewlineType};
/// Editor::new("\n--".to_string(), 1, NewlineType::Lf);
/// ```
/// Changes to:
///
/// ```rust
/// # use linurgy::factory;
/// factory::appender("--", 1);
/// ```
///
/// # Examples
///
/// Extra line
///
/// ```rust
/// # use linurgy::{Editor, NewlineType};
/// let editor = Editor::new("\n\n".to_string(), 1, NewlineType::Lf);
///
/// let output = editor.edit("foo\nbar");
///
/// assert_eq!("foo\n\nbar", output);
/// ```
///
/// ```rust
/// # use linurgy::factory;
/// let output = factory::appender("\n", 1).edit("foo\nbar");
/// assert_eq!("foo\n\nbar", output);
/// ```
///
/// Insert dashes every double newline
///
/// ```rust
/// # use linurgy::{Editor, NewlineType};
/// let editor = Editor::new("-----\n\n".to_string(), 2, NewlineType::Lf);
///
/// let output = editor.edit("foo\n\nbar");
///
/// assert_eq!("foo-----\n\nbar", output);
/// ```
///
/// ```rust
/// # use linurgy::factory;
/// let output = factory::inserter("-----", 2).edit("foo\n\nbar");
/// assert_eq!("foo-----\n\nbar", output);
/// ```
///
/// Replace crlf newlines with tabs
///
/// ```rust
/// # use linurgy::{Editor, NewlineType};
/// let editor = Editor::new("\t".to_string(), 1, NewlineType::Crlf);
///
/// let output = editor.edit("foo\r\nbar");
///
/// assert_eq!("foo\tbar", output);
/// ```
///
/// ```rust
/// # use linurgy::factory;
/// let output = factory::replacer_crlf("\t", 1).edit("foo\r\nbar");
/// assert_eq!("foo\tbar", output);
/// ```
///
/// Extra line buffered version
///
/// ```rust
/// # use linurgy::{Editor, NewlineType};
/// #
/// # use std::io::{BufReader, Result};
/// # fn main() -> Result<()> {
/// let editor = Editor::new("\n\n".to_string(), 1, NewlineType::Lf);
///
/// let mut input = BufReader::new("foo\nbar".as_bytes());
///
/// let mut output = Vec::<u8>::new();
///
/// editor.edit_buffered(&mut input, &mut output)?;
///
/// assert_eq!("foo\n\nbar", String::from_utf8_lossy(&output));
/// #
/// # Ok(())
/// # }
/// ```
///
/// # Default
///
/// [`Editor::default`] returns an editor which makes no changes to input text.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Editor {
    replace: String,
    newlines: u8,
    line_ending: NewlineType,
}

/// The two types of
/// [newline](https://en.wikipedia.org/wiki/Newline#Issues_with_different_newline_formats).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NewlineType {
    /// Line ending: `\n`
    Lf,

    /// Line ending: `\r\n`
    Crlf,
}
