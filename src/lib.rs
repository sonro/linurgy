pub enum Input {
    StdIn,
    File(String),
    Buffer(String),
}

pub enum Output<'b> {
    StdOut,
    File(String),
    Buffer(String),
}

pub struct Editor {
    newline_count_trigger: u8,
    new_text: String,
    current_count: u8,
    buffer: String
}

impl Default for Editor {
    fn default() -> Self {
        Editor {
            newline_count_trigger: 2,
            new_text: String::From("-------\n"),
            current_count: 0,
            buffer: String::new(),
        }
    }
}

pub struct LinurgyBuilder {
    input:  Input,
    output: Output,
    editor: Editor,
}

impl Default for LinurgyBuilder {
    fn default() -> Self {
        LinurgyBuilder {
            input: Input::StdIn,
            output: Output::StdOut,
            editor: Editor::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_editor() {
        let editor = Editor::Default();
        assert_eq!(2, editor.newline_count_trigger);
        assert_eq!(0, editor.current_count);
        assert_eq!("", editor.buffer);
        assert_eq!("-------\n", editor.new_text);
    }
}
