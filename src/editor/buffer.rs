use super::NewlineType;
use std::io::{BufRead, Write};

const BUFSIZE: usize = 1024;

#[derive(Debug)]
pub struct Editor<'a, I, O>
where
    I: BufRead,
    O: Write,
{
    replace: String,
    trigger: u8,
    newline: NewlineType,
    input: &'a mut I,
    output: &'a mut O,
}

impl<'a, I, O> Editor<'a, I, O>
where
    I: BufRead,
    O: Write,
{
    #[inline]
    fn edit(&mut self) -> Result<(), std::io::Error> {
        let mut newlines = 0;
        let mut buf = String::with_capacity(BUFSIZE);

        let (newline_len, newline_str) = match self.newline {
            NewlineType::Lf => (1, "\n"),
            NewlineType::Crlf => (2, "\r\n"),
        };

        loop {
            buf.clear();

            match self.input.read_line(&mut buf)? {
                // EOF
                0 => break,
                // newline by itself
                len if len == newline_len => {
                    newlines += 1;
                }
                // single newline
                len => {
                    while newlines > 0 {
                        self.output.write_all(newline_str.as_bytes())?;
                        newlines -= 1;
                    }
                    if buf.ends_with('\n') {
                        newlines += 1;
                        buf.truncate(len - newline_len);
                    }
                    self.output.write_all(buf.as_bytes())?;
                }
            }

            if newlines == self.trigger {
                self.output.write_all(self.replace.as_bytes())?;
                newlines = 0;
            }
        }

        // trailing newlines
        while newlines > 0 {
            self.output.write_all(newline_str.as_bytes())?;
            newlines -= 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::tests::{EditTest, EDIT_TESTS};
    use std::io::{BufReader, Cursor};

    #[test]
    fn edit() {
        for test in EDIT_TESTS {
            assert_edit(test)
        }
    }

    fn assert_edit(test: &EditTest) {
        let mut input = BufReader::new(test.input.as_bytes());

        let mut output = Cursor::new(Vec::new());

        let mut editor = Editor {
            replace: test.replace.to_string(),
            trigger: test.trigger,
            newline: test.newline,
            input: &mut input,
            output: &mut output,
        };

        editor.edit().unwrap();

        let output = output.into_inner();

        let actual = String::from_utf8_lossy(&output);

        assert_eq!(test.expected, actual, "\ntest: {}\n", test.name);
    }
}
