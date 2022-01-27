#![allow(dead_code)]

use std::fmt;

mod basic;
mod buffer;
#[cfg(test)]
mod tests;

pub use basic::Editor;
pub use buffer::Editor as BufferEditor;

/// Which action to implement when editing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditType {
    /// New edits will appear after newlines
    Append,

    /// New edits will appear before newlines
    Insert,

    /// New edits will appear instead of newlines
    Replace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NewlineType {
    Lf,
    Crlf,
}

impl NewlineType {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            NewlineType::Lf => "\n",
            NewlineType::Crlf => "\r\n",
        }
    }
}

impl fmt::Display for NewlineType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
