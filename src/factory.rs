/*!
Convenience functions for creating a configuired [`Editor`]. Variations are based on the desired
type of edit: append, insert, or replace. Each has a [`CRLF`](NewlineType#variant.Crlf) version.

# Examples

Using factory function

```rust
# use linurgy::factory;
let editor = factory::appender("---", 2);
let output = editor.edit("foo\n\nbar");
assert_eq!("foo\n\n---bar", output);
```

Creating manually

```rust
# use linurgy::{Editor, NewlineType};
let editor = Editor::new(String::from("\n\n---"), 2, NewlineType::Lf);
let output = editor.edit("foo\n\nbar");
assert_eq!("foo\n\n---bar", output);
```
*/
use crate::{Editor, NewlineType};

/// Create an [`Editor`] that appends text *after* newlines.
#[inline]
pub fn appender(text: &str, newlines: u8) -> Editor {
    Factory::build(text, newlines, EditType::Append, NewlineType::Lf)
}

/// Create an [`Editor`] that inserts text *before* newlines.
#[inline]
pub fn inserter(text: &str, newlines: u8) -> Editor {
    Factory::build(text, newlines, EditType::Insert, NewlineType::Lf)
}

/// Create an [`Editor`] that replaces newlines with given text.
#[inline]
pub fn replacer(text: &str, newlines: u8) -> Editor {
    Factory::build(text, newlines, EditType::Replace, NewlineType::Lf)
}

/// Create an [`Editor`] that appends text *after* CRLF newlines.
#[inline]
pub fn appender_crlf(text: &str, newlines: u8) -> Editor {
    Factory::build(text, newlines, EditType::Append, NewlineType::Crlf)
}

/// Create an [`Editor`] that inserts text *before* CRLF newlines.
#[inline]
pub fn inserter_crlf(text: &str, newlines: u8) -> Editor {
    Factory::build(text, newlines, EditType::Insert, NewlineType::Crlf)
}

/// Create an [`Editor`] that replaces CRLF newlines with given text.
#[inline]
pub fn replacer_crlf(text: &str, newlines: u8) -> Editor {
    Factory::build(text, newlines, EditType::Replace, NewlineType::Crlf)
}

#[derive(Debug)]
struct Factory<'a> {
    /// Text to replace/insert/append.
    text: &'a str,

    /// Type of edit to make to newlines.
    edit_type: EditType,

    /// Line ending type.
    newline: NewlineType,

    /// Number of newlines to trigger replacement.
    trigger: u8,
}

/// Which action to implement when editing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EditType {
    /// New edits will appear after newlines
    Append,

    /// New edits will appear before newlines
    Insert,

    /// New edits will appear instead of newlines
    Replace,
}

impl<'a> Factory<'a> {
    #[inline]
    fn build(text: &'a str, trigger: u8, edit_type: EditType, newline: NewlineType) -> Editor {
        let factory = Self {
            text,
            trigger,
            edit_type,
            newline,
        };

        factory.create_editor()
    }

    #[inline]
    fn create_editor(&self) -> Editor {
        let replace = match self.edit_type {
            EditType::Append => self.append_string(),
            EditType::Insert => self.insert_string(),
            EditType::Replace => String::from(self.text),
        };

        Editor::new(replace, self.trigger, self.newline)
    }

    #[inline]
    fn append_string(&self) -> String {
        let mut replace = self.string_with_replace_capacity();

        for _ in 0..self.trigger {
            replace.push_str(self.newline.as_str());
        }

        replace.push_str(self.text);

        replace
    }

    #[inline]
    fn insert_string(&self) -> String {
        let mut replace = self.string_with_replace_capacity();

        replace.push_str(self.text);

        for _ in 0..self.trigger {
            replace.push_str(self.newline.as_str());
        }

        replace
    }

    #[inline]
    fn string_with_replace_capacity(&self) -> String {
        let capacity = self.text.len() + self.trigger as usize * self.newline.as_str().len();
        String::with_capacity(capacity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appender_blank() {
        let editor = appender("", 0);
        let expected = blank_editor();
        assert_eq!(expected, editor);
    }

    #[test]
    fn inserter_blank() {
        let editor = inserter("", 0);
        let expected = blank_editor();
        assert_eq!(expected, editor);
    }

    #[test]
    fn replacer_blank() {
        let editor = replacer("", 0);
        let expected = blank_editor();
        assert_eq!(expected, editor);
    }

    #[test]
    fn appender_crlf_blank() {
        let editor = appender_crlf("", 0);
        let expected = blank_editor_crlf();
        assert_eq!(expected, editor);
    }

    #[test]
    fn inserter_crlf_blank() {
        let editor = inserter_crlf("", 0);
        let expected = blank_editor_crlf();
        assert_eq!(expected, editor);
    }

    #[test]
    fn replacer_crlf_blank() {
        let editor = replacer_crlf("", 0);
        let expected = blank_editor_crlf();
        assert_eq!(expected, editor);
    }

    #[test]
    fn appender_dash_one_line() {
        let editor = appender("-", 1);
        let expected = Editor::new(String::from("\n-"), 1, NewlineType::Lf);
        assert_eq!(expected, editor);
    }

    #[test]
    fn inserter_dash_one_line() {
        let editor = inserter("-", 1);
        let expected = Editor::new(String::from("-\n"), 1, NewlineType::Lf);
        assert_eq!(expected, editor);
    }

    #[test]
    fn replacer_dash_one_line() {
        let editor = replacer("-", 1);
        let expected = Editor::new(String::from("-"), 1, NewlineType::Lf);
        assert_eq!(expected, editor);
    }

    #[test]
    fn appender_crlf_dash_one_line() {
        let editor = appender_crlf("-", 1);
        let expected = Editor::new(String::from("\r\n-"), 1, NewlineType::Crlf);
        assert_eq!(expected, editor);
    }

    #[test]
    fn inserter_crlf_dash_one_line() {
        let editor = inserter_crlf("-", 1);
        let expected = Editor::new(String::from("-\r\n"), 1, NewlineType::Crlf);
        assert_eq!(expected, editor);
    }

    #[test]
    fn replacer_crlf_dash_one_line() {
        let editor = replacer_crlf("-", 1);
        let expected = Editor::new(String::from("-"), 1, NewlineType::Crlf);
        assert_eq!(expected, editor);
    }

    #[test]
    fn appender_dash_two_lines() {
        let editor = appender("-", 2);
        let expected = Editor::new(String::from("\n\n-"), 2, NewlineType::Lf);
        assert_eq!(expected, editor);
    }

    #[test]
    fn inserter_dash_two_lines() {
        let editor = inserter("-", 2);
        let expected = Editor::new(String::from("-\n\n"), 2, NewlineType::Lf);
        assert_eq!(expected, editor);
    }

    #[test]
    fn replacer_dash_two_lines() {
        let editor = replacer("-", 2);
        let expected = Editor::new(String::from("-"), 2, NewlineType::Lf);
        assert_eq!(expected, editor);
    }

    #[test]
    fn appender_crlf_dash_two_lines() {
        let editor = appender_crlf("-", 2);
        let expected = Editor::new(String::from("\r\n\r\n-"), 2, NewlineType::Crlf);
        assert_eq!(expected, editor);
    }

    #[test]
    fn inserter_crlf_dash_two_lines() {
        let editor = inserter_crlf("-", 2);
        let expected = Editor::new(String::from("-\r\n\r\n"), 2, NewlineType::Crlf);
        assert_eq!(expected, editor);
    }

    #[test]
    fn replacer_crlf_dash_two_lines() {
        let editor = replacer_crlf("-", 2);
        let expected = Editor::new(String::from("-"), 2, NewlineType::Crlf);
        assert_eq!(expected, editor);
    }

    fn blank_editor() -> Editor {
        Editor::new(String::from(""), 0, NewlineType::Lf)
    }

    fn blank_editor_crlf() -> Editor {
        Editor::new(String::from(""), 0, NewlineType::Crlf)
    }
}
