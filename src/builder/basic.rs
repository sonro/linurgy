#![allow(dead_code, clippy::derivable_impls)]
use super::build_macro::impl_common_builder;
use crate::{
    editor::{BasicEditor, NewlineType},
    EditType,
};

#[derive(Debug, PartialEq)]
pub struct EditorBuilder<'a> {
    replace: String,
    trigger: u8,
    newline: NewlineType,
    text: &'a str,
    edit_type: EditType,
    dirty: bool,
}

impl<'a> Default for EditorBuilder<'a> {
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&mut self) -> BasicEditor {
        if self.dirty {
            self.prepare();
        }
        BasicEditor::new(&self.replace, self.trigger, self.newline)
    }

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
