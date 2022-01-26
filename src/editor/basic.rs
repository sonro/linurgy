use super::NewlineType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Editor<'a> {
    replace: &'a str,
    trigger: u8,
    newline: NewlineType,
}

impl<'a> Default for Editor<'a> {
    /// Will do nothing on `edit`
    fn default() -> Self {
        Editor {
            replace: "",
            trigger: 0,
            newline: NewlineType::Lf,
        }
    }
}

impl<'a> Editor<'a> {
    #[inline]
    pub fn new(replace: &'a str, trigger: u8, newline: NewlineType) -> Self {
        Editor {
            replace,
            trigger,
            newline,
        }
    }

    #[inline]
    pub fn edit(&self, input: &str) -> String {
        match self.newline {
            NewlineType::Lf => self.edit_lf(input),
            NewlineType::Crlf => self.edit_crlf(input),
        }
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
        let mut newlines = 0;

        for c in input.chars() {
            newlines = match c {
                '\r' => newlines,
                '\n' => self.handle_newline(&mut output, newlines),
                c => self.handle_char_crlf(&mut output, c, newlines),
            }
        }

        for _ in 0..newlines {
            output.push_str("\r\n");
        }

        output
    }

    #[inline(always)]
    fn handle_newline(&self, output: &mut String, mut newlines: u8) -> u8 {
        newlines += 1;

        if newlines == self.trigger {
            output.push_str(self.replace);
            0
        } else {
            newlines
        }
    }

    #[inline(always)]
    fn handle_char_lf(&self, output: &mut String, c: char, newlines: u8) -> u8 {
        for _ in 0..newlines {
            output.push('\n');
        }
        output.push(c);
        0
    }

    #[inline(always)]
    fn handle_char_crlf(&self, output: &mut String, c: char, newlines: u8) -> u8 {
        for _ in 0..newlines {
            output.push_str("\r\n");
        }
        output.push(c);
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::tests::{EditTest, EDIT_TESTS};

    #[test]
    fn edit() {
        for test in EDIT_TESTS {
            assert_edit(test)
        }
    }

    fn assert_edit(test: &EditTest) {
        let editor = Editor::new(test.replace, test.trigger, test.newline);
        assert_eq!(
            test.expected,
            editor.edit(test.input),
            "\ntest: {}\n",
            test.name
        );
    }
}
