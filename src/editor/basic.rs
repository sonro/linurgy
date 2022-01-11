use super::NewlineType;

pub struct Editor {
    replace: String,
    trigger: u8,
    newline: NewlineType,
}

impl Editor {
    #[inline]
    pub fn edit(&self, input: &str) -> String {
        match self.newline {
            NewlineType::Lf => self.edit_lf(input),
            NewlineType::Crlf => self.edit_crlf(input),
        }
    }
}

impl Editor {
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

        output
    }

    #[inline(always)]
    fn handle_newline(&self, output: &mut String, mut newlines: u8) -> u8 {
        newlines += 1;

        if newlines == self.trigger {
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

    #[test]
    fn edit_append_on_one_newline() {
        assert_edit("foo\n\nbar\n\nbaz\n\n", "foo\nbar\nbaz\n", 1, "\n\n");
    }

    #[test]
    fn edit_windows_insert_on_one_newline() {
        assert_edit_windows(
            "foo\r\n\r\nbar\r\n\r\nbaz\r\n\r\n",
            "foo\r\nbar\r\nbaz\r\n",
            1,
            "\r\n\r\n",
        );
    }

    #[test]
    fn edit_append_dash_on_two_newlines() {
        assert_edit("foo\n\n-bar\nbaz\n\n-", "foo\n\nbar\nbaz\n\n", 2, "\n\n-");
    }

    #[test]
    fn edit_windows_replace_with_dashes_on_three_newlines() {
        assert_edit_windows(
            "foo\r\n\r\nbar\r\nbaz\r\n-----",
            "foo\r\n\r\nbar\r\nbaz\r\n\r\n\r\n",
            3,
            "\r\n-----",
        );
    }

    #[test]
    fn edit_remove_newlines() {
        assert_edit("foobarbaz", "foo\nbar\nbaz\n", 1, "");
    }

    #[test]
    fn edit_zero_trigger_does_nothing() {
        assert_edit("foo", "foo", 0, " ");
    }

    fn assert_edit(expected: &str, input: &str, trigger: u8, replace: &str) {
        let editor = Editor::new_lf(trigger, replace);
        assert_eq!(expected, editor.edit(input));
    }

    fn assert_edit_windows(expected: &str, input: &str, trigger: u8, replace: &str) {
        let editor = Editor::new_crlf(trigger, replace);
        assert_eq!(expected, editor.edit(input));
    }

    impl Editor {
        fn new_lf(trigger: u8, replace: &str) -> Editor {
            Editor {
                trigger,
                replace: replace.to_string(),
                newline: NewlineType::Lf,
            }
        }

        fn new_crlf(trigger: u8, replace: &str) -> Editor {
            Editor {
                trigger,
                replace: replace.to_string(),
                newline: NewlineType::Crlf,
            }
        }
    }
}
