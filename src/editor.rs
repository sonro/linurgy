use std::fmt;
use std::io::{self, BufRead, Write};

use crate::{Editor, NewlineType};

const BUFSIZE: usize = 1024;

impl Editor {
    /// Create a new editor
    ///
    /// - `replace`: string to replace newlines with.
    /// - `newlines`: number of newlines to trigger the replacement.
    /// - `line_ending`: type of newline to use.
    ///
    /// # Examples
    ///
    /// This editor replaces newlines with dashes:
    ///
    /// ```rust
    /// # use linurgy::{Editor, NewlineType};
    /// let editor = Editor::new("\n-".to_string(), 1, NewlineType::Lf);
    /// ```
    ///
    /// This editor will remove double newlines from CRLF text:
    ///
    /// ```rust
    /// # use linurgy::{Editor, NewlineType};
    /// let editor = Editor::new("\r\n".to_string(), 2, NewlineType::Crlf);
    /// ```
    ///
    /// # Factory
    ///
    /// Users of this library are encouraged to use the [`factory`](crate::factory)
    /// functions. These provide convient ways to create instances of this type.
    #[inline]
    pub fn new(replace: String, newlines: u8, line_ending: NewlineType) -> Self {
        Editor {
            replace,
            newlines,
            line_ending,
        }
    }

    /// Edit the input's newlines
    ///
    /// Produces a [`String`] containing the edited text according to how this
    /// editor was constructed. Can be used multiple times. The `replace`
    /// string is used to replace newlines when the `newlines` trigger is met.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use linurgy::{Editor, NewlineType};
    /// let editor = Editor::new("-".to_string(), 1, NewlineType::Lf);
    /// let output = editor.edit("foo\nbar");
    /// assert_eq!("foo-bar", output);
    /// ```
    #[inline]
    pub fn edit(&self, input: &str) -> String {
        match self.line_ending {
            NewlineType::Lf => self.edit_lf(input),
            NewlineType::Crlf => self.edit_crlf(input),
        }
    }

    /// Edit the input buffer's newlines into the output writer
    ///
    /// Input types must implement [`BufRead`].
    /// Output types must implement [`Write`].
    ///
    /// Text is edited according to how this editor was constructed. Can be
    /// used multiple times. The `replace` string is used to replace newlines
    /// when the `newlines` trigger is met.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use std::io::Cursor;
    /// # use std::str::from_utf8;
    /// # use linurgy::{Editor, NewlineType};
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let editor = Editor::new("-".to_string(), 1, NewlineType::Lf);
    /// // Cursor implements BufRead over a string
    /// let mut input = Cursor::new("foo\nbar");
    /// let mut output = Vec::new();
    /// editor.edit_buffered(&mut input, &mut output)?;
    /// assert_eq!("foo-bar", from_utf8(&output)?);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn edit_buffered<I, O>(&self, input: &mut I, output: &mut O) -> Result<(), io::Error>
    where
        I: BufRead,
        O: Write,
    {
        let mut newlines = 0;
        let mut buf = String::with_capacity(BUFSIZE);

        let (newline_len, newline_str) = match self.line_ending {
            NewlineType::Lf => (1, "\n"),
            NewlineType::Crlf => (2, "\r\n"),
        };

        loop {
            buf.clear();

            match input.read_line(&mut buf)? {
                // EOF
                0 => break,
                // newline by itself
                len if len == newline_len => {
                    newlines += 1;
                }
                // single newline
                len => {
                    while newlines > 0 {
                        output.write_all(newline_str.as_bytes())?;
                        newlines -= 1;
                    }
                    if buf.ends_with('\n') {
                        newlines += 1;
                        buf.truncate(len - newline_len);
                    }
                    output.write_all(buf.as_bytes())?;
                }
            }

            if newlines == self.newlines {
                output.write_all(self.replace.as_bytes())?;
                newlines = 0;
            }
        }

        // trailing newlines
        while newlines > 0 {
            output.write_all(newline_str.as_bytes())?;
            newlines -= 1;
        }

        Ok(())
    }

    #[inline]
    fn edit_lf(&self, input: &str) -> String {
        let mut output = String::with_capacity(input.len() + self.replace.len());
        let mut newlines = 0;

        for c in input.chars() {
            newlines = match c {
                '\n' => self.handle_newline(&mut output, newlines),
                c => self.handle_char_lf(&mut output, c, newlines),
            }
        }

        for _ in 0..newlines {
            output.push('\n');
        }

        output
    }

    #[inline]
    fn edit_crlf(&self, input: &str) -> String {
        let mut output = String::with_capacity(input.len() + self.replace.len());
        let mut nl_count = 0;

        for c in input.chars() {
            nl_count = match c {
                '\r' => nl_count,
                '\n' => self.handle_newline(&mut output, nl_count),
                c => self.handle_char_crlf(&mut output, c, nl_count),
            }
        }

        for _ in 0..nl_count {
            output.push_str("\r\n");
        }

        output
    }

    #[inline(always)]
    fn handle_newline(&self, output: &mut String, mut nl_count: u8) -> u8 {
        nl_count += 1;

        if nl_count == self.newlines {
            output.push_str(&self.replace);
            0
        } else {
            nl_count
        }
    }

    #[inline(always)]
    fn handle_char_lf(&self, output: &mut String, c: char, nl_count: u8) -> u8 {
        for _ in 0..nl_count {
            output.push('\n');
        }
        output.push(c);
        0
    }

    #[inline(always)]
    fn handle_char_crlf(&self, output: &mut String, c: char, nl_count: u8) -> u8 {
        for _ in 0..nl_count {
            output.push_str("\r\n");
        }
        output.push(c);
        0
    }
}

impl Default for Editor {
    /// Will do nothing on `edit`
    fn default() -> Self {
        Editor {
            replace: String::new(),
            newlines: 0,
            line_ending: NewlineType::Lf,
        }
    }
}

impl NewlineType {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            NewlineType::Lf => "\n",
            NewlineType::Crlf => "\r\n",
        }
    }
}

impl fmt::Display for NewlineType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod standard {
        use super::*;

        editor_tests!(assert_edit);

        fn assert_edit(test: EditTest) {
            let replace = test.replace.to_string();
            let editor = Editor::new(replace, test.newlines, test.line_ending);

            assert_eq!(
                test.expected,
                editor.edit(test.input),
                "\ntest: {}\n",
                test.name
            );
        }
    }

    mod buffered {
        use super::*;
        use std::io::BufReader;

        editor_tests!(assert_edit_buffered);

        fn assert_edit_buffered(test: EditTest) {
            let mut input = BufReader::new(test.input.as_bytes());

            let mut output: Vec<u8> = Vec::new();

            let replace = test.replace.to_string();
            let editor = Editor::new(replace, test.newlines, test.line_ending);

            editor.edit_buffered(&mut input, &mut output).unwrap();

            let actual = String::from_utf8_lossy(&output);

            assert_eq!(test.expected, actual, "\ntest: {}\n", test.name);
        }
    }

    struct EditTest {
        pub name: &'static str,
        pub expected: &'static str,
        pub input: &'static str,
        pub newlines: u8,
        pub replace: &'static str,
        pub line_ending: NewlineType,
    }

    macro_rules! editor_tests {
        ($assert_fn:ident) => {
            #[test]
            fn leading_newline_preserved() {
                $assert_fn(EditTest {
                    name: "leading newline preserved",
                    expected: "\nfoo\nbar\nbaz\n",
                    input: "\nfoo\nbar\nbaz\n",
                    newlines: 2,
                    replace: "",
                    line_ending: NewlineType::Lf,
                });
            }

            #[test]
            fn leading_newline_preserved_crlf() {
                $assert_fn(EditTest {
                    name: "leading newline preserved crlf",
                    expected: "\r\nfoo\r\nbar\r\nbaz\r\n",
                    input: "\r\nfoo\r\nbar\r\nbaz\r\n",
                    newlines: 2,
                    replace: "",
                    line_ending: NewlineType::Crlf,
                });
            }

            #[test]
            fn no_trailing_newline_preserved() {
                $assert_fn(EditTest {
                    name: "no trailing newline preserved",
                    expected: "foo\nbar\nbaz",
                    input: "foo\nbar\nbaz",
                    newlines: 2,
                    replace: "",
                    line_ending: NewlineType::Lf,
                });
            }

            #[test]
            fn no_trailing_newline_preserved_crlf() {
                $assert_fn(EditTest {
                    name: "no trailing newline preserved crlf",
                    expected: "foo\r\nbar\r\nbaz",
                    input: "foo\r\nbar\r\nbaz",
                    newlines: 2,
                    replace: "",
                    line_ending: NewlineType::Crlf,
                });
            }

            #[test]
            fn insert_dash_every_line() {
                $assert_fn(EditTest {
                    name: "insert dash every line",
                    expected: "foo-\nbar-\nbaz-\n",
                    input: "foo\nbar\nbaz\n",
                    newlines: 1,
                    replace: "-\n",
                    line_ending: NewlineType::Lf,
                });
            }

            #[test]
            fn insert_dash_every_line_crlf() {
                $assert_fn(EditTest {
                    name: "insert dash every line crlf",
                    expected: "foo-\r\nbar-\r\nbaz-\r\n",
                    input: "foo\r\nbar\r\nbaz\r\n",
                    newlines: 1,
                    replace: "-\r\n",
                    line_ending: NewlineType::Crlf,
                });
            }

            #[test]
            fn append_dash_every_line() {
                $assert_fn(EditTest {
                    name: "append dash every line",
                    expected: "foo\n-bar\n-baz\n-",
                    input: "foo\nbar\nbaz\n",
                    newlines: 1,
                    replace: "\n-",
                    line_ending: NewlineType::Lf,
                });
            }

            #[test]
            fn append_dash_every_line_crlf() {
                $assert_fn(EditTest {
                    name: "append dash every line crlf",
                    expected: "foo\r\n-bar\r\n-baz\r\n-",
                    input: "foo\r\nbar\r\nbaz\r\n",
                    newlines: 1,
                    replace: "\r\n-",
                    line_ending: NewlineType::Crlf,
                });
            }

            #[test]
            fn replace_with_dash_every_line() {
                $assert_fn(EditTest {
                    name: "replace with dash every line",
                    expected: "foo-bar-baz",
                    input: "foo\nbar\nbaz",
                    newlines: 1,
                    replace: "-",
                    line_ending: NewlineType::Lf,
                });
            }

            #[test]
            fn replace_with_dash_every_line_crlf() {
                $assert_fn(EditTest {
                    name: "replace with dash every line crlf",
                    expected: "foo-bar-baz",
                    input: "foo\r\nbar\r\nbaz",
                    newlines: 1,
                    replace: "-",
                    line_ending: NewlineType::Crlf,
                });
            }

            #[test]
            fn append_extra_line() {
                $assert_fn(EditTest {
                    name: "append extra line",
                    expected: "foo\n\nbar\n\nbaz\n\n",
                    input: "foo\nbar\nbaz\n",
                    newlines: 1,
                    replace: "\n\n",
                    line_ending: NewlineType::Lf,
                });
            }

            #[test]
            fn append_extra_line_crlf() {
                $assert_fn(EditTest {
                    name: "append extra line crlf",
                    expected: "foo\r\n\r\nbar\r\n\r\nbaz\r\n\r\n",
                    input: "foo\r\nbar\r\nbaz\r\n",
                    newlines: 1,
                    replace: "\r\n\r\n",
                    line_ending: NewlineType::Crlf,
                });
            }

            #[test]
            fn remove_extra_line() {
                $assert_fn(EditTest {
                    name: "remove extra line",
                    expected: "foo\nbar\nbaz\n",
                    input: "foo\n\nbar\n\nbaz\n\n",
                    newlines: 2,
                    replace: "\n",
                    line_ending: NewlineType::Lf,
                });
            }

            #[test]
            fn remove_extra_line_crlf() {
                $assert_fn(EditTest {
                    name: "remove extra line crlf",
                    expected: "foo\r\nbar\r\nbaz\r\n",
                    input: "foo\r\n\r\nbar\r\n\r\nbaz\r\n\r\n",
                    newlines: 2,
                    replace: "\r\n",
                    line_ending: NewlineType::Crlf,
                });
            }

            #[test]
            fn zero_newlines_does_nothing() {
                $assert_fn(EditTest {
                    name: "zero newlines does nothing",
                    expected: "foo\nbar\n\nbaz\n\n\n",
                    input: "foo\nbar\n\nbaz\n\n\n",
                    newlines: 0,
                    replace: "should not be used",
                    line_ending: NewlineType::Lf,
                });
            }

            #[test]
            fn zero_newlines_does_nothing_crlf() {
                $assert_fn(EditTest {
                    name: "zero newlines does nothing crlf",
                    expected: "foo\r\nbar\r\n\r\nbaz\r\n\r\n\r\n",
                    input: "foo\r\nbar\r\n\r\nbaz\r\n\r\n\r\n",
                    newlines: 0,
                    replace: "should not be used",
                    line_ending: NewlineType::Crlf,
                });
            }

            #[test]
            fn insert_dash_every_5_lines() {
                $assert_fn(EditTest {
                    name: "insert dash every 5 lines",
                    expected: "foo-\n\n\n\n\n-\n\n\n\n\n",
                    input: "foo\n\n\n\n\n\n\n\n\n\n",
                    newlines: 5,
                    replace: "-\n\n\n\n\n",
                    line_ending: NewlineType::Lf,
                });
            }

            #[test]
            fn insert_dash_every_4_lines_crlf() {
                $assert_fn(EditTest {
                    name: "insert dash every 4 lines crlf",
                    expected: "foo-\r\n\r\n\r\n\r\n-\r\n\r\n\r\n\r\n",
                    input: "foo\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n",
                    newlines: 4,
                    replace: "-\r\n\r\n\r\n\r\n",
                    line_ending: NewlineType::Crlf,
                });
            }

            #[test]
            fn replace_dash_every_3_lines() {
                $assert_fn(EditTest {
                    name: "replace dash every 3 lines",
                    expected: "foo-bar-baz",
                    input: "foo\n\n\nbar\n\n\nbaz",
                    newlines: 3,
                    replace: "-",
                    line_ending: NewlineType::Lf,
                });
            }
        };
    }

    pub(super) use editor_tests;
}
