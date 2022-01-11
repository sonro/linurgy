#![allow(dead_code)]

use std::fmt;

mod basic;

/// Which action to implement when editing
#[derive(PartialEq, Debug)]
pub enum EditType {
    /// New edits will appear after newlines
    Append,

    /// New edits will appear before newlines
    Insert,

    /// New edits will appear instead of newlines
    Replace,
}

pub enum NewlineType {
    Lf,
    Crlf,
}

impl NewlineType {
    #[inline]
    fn newline_str(&self) -> &'static str {
        match self {
            NewlineType::Lf => "\n",
            NewlineType::Crlf => "\r\n",
        }
    }
}

impl fmt::Display for NewlineType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.newline_str())
    }
}

pub struct StreamEditor;

pub struct StdioEditor;

pub struct FileEditor;
