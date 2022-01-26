use super::build_macro::impl_common_builder;
use crate::{
    editor::{BasicEditor, NewlineType},
    EditType,
};

/// Configures, prepares, and builds [`BasicEditor`].
#[derive(Debug, PartialEq)]
pub struct EditorBuilder<'a> {
    /// Prepared string to replace newlines with.
    replace: String,

    /// Number of newlines to trigger replacement.
    trigger: u8,

    /// Line ending type.
    newline: NewlineType,

    /// Text to replace/insert/append.
    text: &'a str,

    /// Type of edit to make to newlines.
    edit_type: EditType,

    /// Whether this builder has been edited since being prepared.
    dirty: bool,
}

impl<'a> Default for EditorBuilder<'a> {
    /// Will build into default BasicEditor.
    fn default() -> Self {
        Self {
            replace: String::new(),
            trigger: 0,
            newline: NewlineType::Lf,
            text: "",
            edit_type: EditType::Replace,
            dirty: false,
        }
    }
}

impl<'a> EditorBuilder<'a> {
    /// Creates a new builder with following defaults:
    ///
    /// - text: `""`,
    /// - newline_trigger: `0`,
    /// - edit_type: `Replace`,
    ///
    /// The [`BasicEditor`] made by a unaltered builder will make no changes
    /// when editing.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Prepare and build a new [`BasicEditor`] instance.
    ///
    /// This is a convenience method for both preparing and building. If you
    /// are going to be constructing multiple [`BasicEditor`] instances, it
    /// is recommended to use [`prepare`](Self::prepare) and
    /// [`build_prepared`](Self::build_prepared) instead.
    #[inline]
    pub fn build(&mut self) -> BasicEditor {
        if self.dirty {
            self.prepare();
        }
        self.build_prepared()
    }

    /// Build a new [`BasicEditor`] instance.
    ///
    /// This method should only be used after [`prepare`](Self::prepare) has
    /// been called.
    #[inline]
    pub fn build_prepared(&self) -> BasicEditor {
        BasicEditor::new(&self.replace, self.trigger, self.newline)
    }

    impl_common_builder!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_build_same_as_editor_default() {
        let mut builder = EditorBuilder::default();
        assert_eq!(BasicEditor::default(), builder.build());
    }

    #[test]
    fn new_just_creates_default() {
        let default = EditorBuilder::default();
        let new = EditorBuilder::new();
        assert_eq!(default, new);
    }

    #[test]
    fn set_newline_trigger() {
        let mut builder = EditorBuilder::new();
        builder.newline_trigger(2);
        let expected = BasicEditor::new("", 2, NewlineType::Lf);
        assert_eq!(expected, builder.build());
    }

    #[test]
    fn default_edit_type_replace() {
        let default = EditorBuilder::default();
        let mut explicit_replace = EditorBuilder::new();
        explicit_replace.replace();
        assert_eq!(default.edit_type, explicit_replace.edit_type);
    }

    #[test]
    fn replace_with_dashes() {
        let mut builder = EditorBuilder::new();
        builder.text("--");
        let expected = BasicEditor::new("--", 0, NewlineType::Lf);
        assert_eq!(expected, builder.build());
    }

    #[test]
    fn append_with_dashes() {
        let mut builder = EditorBuilder::new();
        builder.text("--").append();
        let expected = BasicEditor::new("\n--", 0, NewlineType::Lf);
        assert_eq!(expected, builder.build());
    }

    #[test]
    fn insert_with_dashes() {
        let mut builder = EditorBuilder::new();
        builder.text("--").insert();
        let expected = BasicEditor::new("--\n", 0, NewlineType::Lf);
        assert_eq!(expected, builder.build());
    }

    #[test]
    fn default_newline_type_lf() {
        let default = EditorBuilder::default();
        let mut explicit_lf = EditorBuilder::new();
        explicit_lf.newline_lf();
        assert_eq!(default.newline, explicit_lf.newline);
    }

    #[test]
    fn crlf_newline_append_with_dashes() {
        let mut builder = EditorBuilder::new();
        builder.newline_crlf().text("--").append();
        let expected = BasicEditor::new("\r\n--", 0, NewlineType::Crlf);
        assert_eq!(expected, builder.build());
    }
}
