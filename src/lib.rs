pub enum Input<'a> {
    StdIn,
    File(String),
    Buffer(&'a String),
}

pub enum Output<'b> {
    StdOut,
    File(String),
    Buffer(&'b mut String),
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
            new_text: String::from("-------\n"),
            current_count: 0,
            buffer: String::new(),
        }
    }
}

pub struct LinurgyBuilder<'a, 'b> {
    input:  Input<'a>,
    output: Output<'b>,
    editor: Editor,
}

impl Default for LinurgyBuilder<'_, '_> {
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
        let editor = Editor::default();
        assert_eq!(2, editor.newline_count_trigger);
        assert_eq!(0, editor.current_count);
        assert_eq!("", editor.buffer);
        assert_eq!("-------\n", editor.new_text);
    }

    #[test]
    fn default_linurgy_builder() {
        let lb = LinurgyBuilder::default();
        let editor = Editor::default();
        if let Input::StdIn = lb.input {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Output::StdOut = lb.output {
            assert!(true);
        } else {
            assert!(false);
        }

        assert_eq!(editor.new_text, lb.editor.new_text);
    }
}
