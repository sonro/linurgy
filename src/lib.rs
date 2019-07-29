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

impl<'a, 'b> LinurgyBuilder<'a, 'b> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_input(&mut self, input: Input<'a>) -> &mut Self {
        self.input = input;
        self
    }

    pub fn add_output(&mut self, output: Output<'b>) -> &mut Self {
        self.output = output;
        self
    }

    pub fn add_newline_trigger(&mut self, count: u8) -> &mut Self {
        self.editor.newline_count_trigger = count;
        self
    }

    pub fn add_new_text(&mut self, new_text: String) -> &mut Self {
        self.editor.new_text = new_text;
        self
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
        let lb = LinurgyBuilder::new();
        let editor = Editor::default();
        if let Input::StdIn = lb.input {
            assert!(true);
        } else {
            assert!(false, "Correct type not implemented");
        }

        if let Output::StdOut = lb.output {
            assert!(true);
        } else {
            assert!(false, "Correct type not implemented");
        }

        assert_eq!(editor.new_text, lb.editor.new_text);
    }

    #[test]
    fn linurgy_add_input() {
        let buffer = String::from("Test builder");
        let mut lb = LinurgyBuilder::new();

        lb.add_input(Input::Buffer(&buffer));
        match lb.input {
            Input::Buffer(text) => assert_eq!(&buffer, text),
            _ => assert!(false, "Correct type not implemented"),
        }
        
        lb.add_input(Input::File(String::from("filename")));
        match lb.input {
            Input::File(ref text) => assert_eq!("filename", text),
            _ => assert!(false, "Correct type not implemented"),
        }

        lb.add_input(Input::StdIn);
        match lb.input {
            Input::StdIn => assert!(true),
            _ => assert!(false, "Correct type not implemented"),
        }
    }

    #[test]
    fn linurgy_add_output() {
        let mut buffer = String::from("Test builder");
        let mut buffer2 = String::from("Test builder");
        let mut lb = LinurgyBuilder::new();

        lb.add_output(Output::Buffer(&mut buffer));
        match lb.output {
            Output::Buffer(ref text) => assert_eq!(&&mut buffer2, text),
            _ => assert!(false, "Correct type not implemented"),
        }
        
        lb.add_output(Output::File(String::from("filename")));
        match lb.output {
            Output::File(ref text) => assert_eq!("filename", text),
            _ => assert!(false, "Correct type not implemented"),
        }

        lb.add_output(Output::StdOut);
        match lb.output {
            Output::StdOut => assert!(true),
            _ => assert!(false, "Correct type not implemented"),
        }
    }

    #[test]
    fn linurgy_add_newline_trigger() {
        let mut lb = LinurgyBuilder::new();
        
        lb.add_newline_trigger(5);
        assert_eq!(5, lb.editor.newline_count_trigger);
    }

    #[test]
    fn linurgy_add_new_text() {
        let mut lb = LinurgyBuilder::new();
        
        lb.add_new_text(String::from("cheese"));
        assert_eq!("cheese", lb.editor.new_text);
    }
}
