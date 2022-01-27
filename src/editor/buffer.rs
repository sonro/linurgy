use super::NewlineType;
use std::io::{BufRead, Write};

const BUFSIZE: usize = 1024;

#[derive(Debug)]
pub struct BufEditor<'i, 'o, I, O>
where
    I: BufRead,
    O: Write,
{
    replace: String,
    newlines: u8,
    line_ending: NewlineType,
    input: &'i mut I,
    output: &'o mut O,
}

impl<'i, 'o, I, O> BufEditor<'i, 'o, I, O>
where
    I: BufRead,
    O: Write,
{
    #[inline]
    fn edit(&mut self) -> Result<(), std::io::Error> {
        let mut newlines = 0;
        let mut buf = String::with_capacity(BUFSIZE);

        let (newline_len, newline_str) = match self.line_ending {
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

            if newlines == self.newlines {
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
    use crate::editor::tests::{editor_tests, EditTest};
    use std::io::BufReader;

    fn assert_edit(test: EditTest) {
        let mut input = BufReader::new(test.input.as_bytes());

        let mut output: Vec<u8> = Vec::new();

        let mut editor = BufEditor {
            replace: test.replace.to_string(),
            newlines: test.newlines,
            line_ending: test.line_ending,
            input: &mut input,
            output: &mut output,
        };

        editor.edit().unwrap();

        let actual = String::from_utf8_lossy(&output);

        assert_eq!(test.expected, actual, "\ntest: {}\n", test.name);
    }

    editor_tests!(assert_edit);
}
