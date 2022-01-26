#![allow(dead_code, clippy::derivable_impls)]
use crate::editor::{BasicEditor, NewlineType};

#[derive(Debug)]
pub struct EditorBuilder {}

impl Default for EditorBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl EditorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&self) -> BasicEditor {
        let replace = "";
        let trigger = 0;
        let newline = NewlineType::Lf;
        BasicEditor::new(replace, trigger, newline)
    }
}
