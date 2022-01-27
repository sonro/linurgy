use super::NewlineType;

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

        if newlines == self.newlines {
            output.push_str(&self.replace);
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
    use crate::editor::tests::{editor_tests, EditTest};

    fn assert_edit(test: EditTest) {
        let replace = test.replace.to_string();
        let editor = Editor::new(replace, test.trigger, test.newline);
        assert_eq!(
            test.expected,
            editor.edit(test.input),
            "\ntest: {}\n",
            test.name
        );
    }

    editor_tests!(assert_edit);
}
