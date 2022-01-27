use super::NewlineType;
use std::io::{self, BufRead, Write};

const BUFSIZE: usize = 1024;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Editor {
    replace: String,
    newlines: u8,
    line_ending: NewlineType,
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

impl Editor {
    #[inline]
    pub fn new(replace: String, newlines: u8, line_ending: NewlineType) -> Self {
        Editor {
            replace,
            newlines,
            line_ending,
        }
    }

    #[inline]
    pub fn edit(&self, input: &str) -> String {
        match self.line_ending {
            NewlineType::Lf => self.edit_lf(input),
            NewlineType::Crlf => self.edit_crlf(input),
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::tests::{editor_tests, EditTest};

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

    mod buffered {
        use super::*;
        use std::io::BufReader;

        editor_tests!(assert_edit);

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
}
