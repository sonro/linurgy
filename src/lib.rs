//! `linurgy` provides an interface for manipulating multiple newlines in text.
//! Interaction with this library happens through 
//! [`LinurgyBuilder`](struct.LinurgyBuilder.html).
//!
//! # Examples
//!
//! Read stdin and for each empty line, append an extra line to stdout.
//! ```rust
//! # use linurgy::LinurgyBuilder;
//! LinurgyBuilder::new()
//!     .set_newline_trigger(1)
//!     .set_new_text(String::from("\n"))
//!     .run();
//! ```
//! 
//! Read from one buffer, remove all empty lines, and output to another buffer.
//! ```rust
//! # use linurgy::{LinurgyBuilder, Input, Output, EditType};
//! let input = String::from("Remove\n\nEvery\n\nEmpty\n\nLine\n");
//! let mut output = String::new();
//! 
//! LinurgyBuilder::new()
//!     .set_input(Input::Buffer(&input))
//!     .set_output(Output::Buffer(&mut output))
//!     .set_newline_trigger(1)
//!     .set_edit_type(EditType::Replace)
//!     .set_new_text(String::from(""))
//!     .run();
//! 
//! assert_eq!("Remove\nEvery\nEmpty\nLine\n", &output);
//! ```

use std::io::{self, Write};
use std::fs;

/// Type of input stream to edit
pub enum Input<'a> {
    /// Basic line by line read from stdin
    StdIn,

    /// Read from a given filename
    File(String),

    /// Read from a string
    Buffer(&'a str),
}

/// Type of output stream to write edits to
pub enum Output<'b> {
    /// Basic line by line output to stdout
    StdOut,

    /// Write to a given filename
    File(String),

    /// Write to a given `String` buffer
    Buffer(&'b mut String),
}

/// Which action to implement when editing newlines
pub enum EditType {
    /// New edits will appear after newlines
    Append,

    /// New edits will appear before newlines
    Insert,

    /// New edits will appear instead of newlines
    Replace,
}

struct Editor {
    newline_count_trigger: u8,
    new_text: String,
    edit_type: EditType,
    current_count: u8,
    buffer: String
}

impl Default for Editor {
    fn default() -> Self {
        Editor {
            newline_count_trigger: 2,
            new_text: String::from("-------\n"),
            edit_type: EditType::Append,
            current_count: 0,
            buffer: String::new(),
        }
    }
}

impl Editor {
    fn add_line(&mut self, line: &str) {
        self.buffer += line;
        if line == "\n" {
            self.current_count += 1;
            if self.current_count == self.newline_count_trigger {
                self.current_count = 0;
                match &self.edit_type {
                    EditType::Append => self.buffer += &self.new_text,
                    EditType::Insert => {
                        self.buffer.insert_str(0, &self.new_text);
                    }
                    EditType::Replace => {
                        self.buffer.replace_range(.., &self.new_text);
                    }
                }
            }
        } else {
            // line contains text
            self.current_count = 0;
        }
    }

    fn try_output(&mut self) -> Option<String> {
        if self.current_count == 0 {
            Some(self.buffer.drain(..).collect())
        } else {
            None
        }
    }

    fn get_remaining_output(&mut self) -> Option<String> {
        if !self.buffer.is_empty() {
            Some(self.buffer.drain(..).collect())
        } else {
            None
        }
    }
}

/// Use this to prepare and execute linurgy editing on a stream.
///
/// A linurgy consists of an [`Input`](enum.Input.html), which will be read
/// line by line, edited by user defined rules, and then streamed into an
/// [`Output`](enum.Output.html).
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
    /// Instantiate a new builder with default values.
    /// - Input: [`Input::StdIn`](enum.Input.html#variant.StdIn),
    /// - Output: [`Output::StdOut`](enum.Output.html#variant.StdOut),
    /// - Newline count trigger: 2,
    /// - New text : "-------\n",
    /// - EditType: [`EditType::Append`](enum.EditType.html#variant.Append)
    ///
    /// This will read from `stdin`,
    /// add dashes after 2 empty lines, and write to `stdout`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the input source to read text from.
    ///
    /// # Examples
    /// Using an in-memory [`Buffer`](enum.Input.html#variant.Buffer)
    /// ```rust
    /// # use linurgy::{LinurgyBuilder, Input};
    /// let text = String::from("Sample text\n\n\n");
    /// let mut linurgy = LinurgyBuilder::new();
    ///
    /// linurgy.set_input(Input::Buffer(&text));
    /// ```
    /// Read from a [`File`](enum.Input.html#variant.File)
    /// ```rust
    /// # use linurgy::{LinurgyBuilder, Input};
    /// let filename = String::from("filename.txt");
    /// let mut linurgy = LinurgyBuilder::new();
    /// 
    /// linurgy.set_input(Input::File(filename));
    /// ```
    pub fn set_input(&mut self, input: Input<'a>) -> &mut Self {
        self.input = input;
        self
    }

    /// Set the output stream to write to.
    ///
    /// # Examples
    /// Using an in-memory [`Buffer`](enum.Output.html#variant.Buffer)
    /// ```rust
    /// # use linurgy::{LinurgyBuilder, Output};
    /// let mut buffer = String::new();
    /// let mut linurgy = LinurgyBuilder::new();
    ///
    /// linurgy.set_output(Output::Buffer(&mut buffer));
    /// ```
    /// Write straight to a [`File`](enum.Output.html#variant.File)
    /// ```rust
    /// # use linurgy::{LinurgyBuilder, Output};
    /// let filename = String::from("filename.txt");
    /// let mut linurgy = LinurgyBuilder::new();
    /// 
    /// linurgy.set_output(Output::File(filename));
    /// ```
    pub fn set_output(&mut self, output: Output<'b>) -> &mut Self {
        self.output = output;
        self
    }

    /// Set the newline count to trigger editing.
    ///
    /// # Example
    /// Add edit string after every 5 empty lines
    /// ```rust
    /// # use linurgy::{LinurgyBuilder};
    /// let mut linurgy = LinurgyBuilder::new();
    /// linurgy.set_newline_trigger(5);
    /// ```
    pub fn set_newline_trigger(&mut self, count: u8) -> &mut Self {
        self.editor.newline_count_trigger = count;
        self
    }

    /// Set the text that will be used when the newline trigger is reached.
    ///
    /// # Example
    /// Add a line of dots after every empty line
    /// ```rust
    /// # use linurgy::{LinurgyBuilder};
    /// let mut linurgy = LinurgyBuilder::new();
    /// linurgy.set_newline_trigger(1);
    /// linurgy.set_new_text(
    ///     format!("{}\n", ". ".repeat(25))
    /// );
    /// ```
    pub fn set_new_text(&mut self, new_text: String) -> &mut Self {
        self.editor.new_text = new_text;
        self
    }

    /// Set how new text is added after the newline trigger is reached.
    ///
    /// # Example
    /// Replace double empty lines with a line of dashes
    /// ```rust
    /// # use linurgy::{LinurgyBuilder, EditType};
    /// let mut linurgy = LinurgyBuilder::new();
    /// linurgy.set_edit_type(EditType::Replace);
    /// ```
    pub fn set_edit_type(&mut self, edit_type: EditType) -> &mut Self {
        self.editor.edit_type = edit_type;
        self
    }

    /// Execute the linurgy edits on the specified input stream.
    ///
    /// This function will block until the input stream is exhausted.
    /// If the input stream is [`Input::StdIn`](enum.Input.html#varient.StdIn),
    /// then `stdin` will be locked while this function runs.
    /// If `stdin` is locked elsewhere, this function will block until it
    /// becomes available again.
    ///
    /// # Panics
    /// This function will panic if the input file in unable to be opened
    /// or read from, 
    /// or if the output file is unable to be created or written to.
    ///
    /// # Examples
    /// Execute default behaviour and add dashes to 
    /// double newlines from `stdin`
    /// ```rust
    /// # use linurgy::LinurgyBuilder;
    /// LinurgyBuilder::new().run();
    /// ```
    ///
    /// This will panic if "not-a-file" does not a exist
    /// ```rust,should_panic
    /// # use linurgy::{LinurgyBuilder, Input};
    /// LinurgyBuilder::new()
    ///     .set_input(Input::File(String::from("not-a-file")))
    ///     .run();
    /// ```
    pub fn run(&mut self) -> &mut Self {
        match self.input {
            Input::StdIn => {
                let stdin = io::stdin();
                let reader = stdin.lock();
                self.process(reader);
            }
            Input::File(ref name) => {
                let file = fs::File::open(name).expect("Unable to open file");
                let reader = io::BufReader::new(file);
                self.process(reader);
            }
            Input::Buffer(buffer) => {
                let reader = io::Cursor::new(buffer);
                self.process(reader);
            }
        }

        self
    }

    fn process(&mut self, reader: impl io::BufRead) {
        let mut buffer = String::new();

        for line in reader.lines() {
            let line = line.unwrap() + "\n";
            self.editor.add_line(&line);
            if let Some(edited) = self.editor.try_output() {
                match self.output {
                    Output::StdOut => print!("{}", &edited),
                    _ => buffer += &edited,
                }
            }
        }

        if let Some(edited) = self.editor.get_remaining_output() {
            match self.output {
                Output::StdOut => print!("{}", &edited),
                _ => buffer += &edited,
            }
        }

        match self.output {
            Output::File(ref name) => {
                let mut file = fs::File::create(name)
                    .expect("Unable to create file");
                file.write_all(buffer.as_bytes())
                    .expect("unable to write to file");
            }
            Output::Buffer(ref mut buf) => {
                buf.push_str(&buffer);
            }
            _ => (),
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
        if let EditType::Append = editor.edit_type {
            assert!(true);
        } else {
            assert!(false, "Correct type not implemented");
        }
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
    fn linurgy_set_input() {
        let buffer = String::from("Test builder");
        let mut lb = LinurgyBuilder::new();

        lb.set_input(Input::Buffer(&buffer));
        match lb.input {
            Input::Buffer(text) => assert_eq!(&buffer, text),
            _ => assert!(false, "Correct type not implemented"),
        }
        
        lb.set_input(Input::File(String::from("filename")));
        match lb.input {
            Input::File(ref text) => assert_eq!("filename", text),
            _ => assert!(false, "Correct type not implemented"),
        }

        lb.set_input(Input::StdIn);
        match lb.input {
            Input::StdIn => assert!(true),
            _ => assert!(false, "Correct type not implemented"),
        }
    }

    #[test]
    fn linurgy_set_output() {
        let mut buffer = String::from("Test builder");
        let mut buffer2 = String::from("Test builder");
        let mut lb = LinurgyBuilder::new();

        lb.set_output(Output::Buffer(&mut buffer));
        match lb.output {
            Output::Buffer(ref text) => assert_eq!(&&mut buffer2, text),
            _ => assert!(false, "Correct type not implemented"),
        }
        
        lb.set_output(Output::File(String::from("filename")));
        match lb.output {
            Output::File(ref text) => assert_eq!("filename", text),
            _ => assert!(false, "Correct type not implemented"),
        }

        lb.set_output(Output::StdOut);
        match lb.output {
            Output::StdOut => assert!(true),
            _ => assert!(false, "Correct type not implemented"),
        }
    }

    #[test]
    fn linurgy_set_newline_trigger() {
        let mut lb = LinurgyBuilder::new();
        
        lb.set_newline_trigger(5);
        assert_eq!(5, lb.editor.newline_count_trigger);
    }

    #[test]
    fn linurgy_set_new_text() {
        let mut lb = LinurgyBuilder::new();
        
        lb.set_new_text(String::from("cheese"));
        assert_eq!("cheese", lb.editor.new_text);
    }

    #[test]
    fn linurgy_set_edit_type() {
        let mut lb = LinurgyBuilder::new();
        
        lb.set_edit_type(EditType::Insert);
        if let EditType::Insert = lb.editor.edit_type {
            assert!(true);
        } else {
            assert!(false, "Correct type not implemented");
        }
    }

    #[test]
    fn editor_add_line() {
        let mut ed = Editor::default();

        let line = String::from("test text\n");
        ed.add_line(&line);
        assert_eq!("test text\n", ed.buffer);
        assert_eq!(0, ed.current_count);

        let line = String::from(" more\n");
        ed.add_line(&line);
        assert_eq!("test text\n more\n", ed.buffer);
        assert_eq!(0, ed.current_count);

        let line = String::from("\n");
        ed.add_line(&line);
        assert_eq!("test text\n more\n\n", ed.buffer);
        assert_eq!(1, ed.current_count);

        let line = String::from("\n");
        ed.add_line(&line);
        assert_eq!("test text\n more\n\n\n-------\n", ed.buffer);
        assert_eq!(0, ed.current_count);
    }

    #[test]
    fn editor_add_line_with_diff_edit_type() {
        let mut ed = Editor::default();
        ed.edit_type = EditType::Insert;

        let line = String::from("\n");
        ed.add_line(&line);
        ed.add_line(&line);
        assert_eq!("-------\n\n\n", ed.buffer);

        let mut ed = Editor::default();
        ed.edit_type = EditType::Replace;

        let line = String::from("\n");
        ed.add_line(&line);
        ed.add_line(&line);
        assert_eq!("-------\n", ed.buffer);
    }

    #[test]
    fn editor_try_output() {
        let mut ed = Editor::default();
        assert_eq!(Some(String::from("")), ed.try_output());

        let line = String::from("\n");
        ed.add_line(&line);
        assert_eq!(None, ed.try_output());

        ed.add_line(&line);
        assert_eq!(Some(String::from("\n\n-------\n")), ed.try_output());
        assert_eq!(Some(String::from("")), ed.try_output());

        let line = String::from("test\n");
        ed.add_line(&line);
        assert_eq!(Some(String::from("test\n")), ed.try_output());
    }

    #[test]
    fn linurgy_process() {
        let mut output = String::new();
        let mut lb = LinurgyBuilder::new();
        lb.set_output(Output::Buffer(&mut output));

        let input = String::from("test\nlines\n");
        let reader = io::Cursor::new(&input);
        lb.process(reader);
        assert_eq!("test\nlines\n", &output);

        let mut output = String::new();
        let mut lb = LinurgyBuilder::new();
        lb.set_output(Output::Buffer(&mut output));

        let input = String::from("\n\n");
        let reader = io::Cursor::new(&input);
        lb.process(reader);
        assert_eq!("\n\n-------\n", &output);

        let mut output = String::new();
        let mut lb = LinurgyBuilder::new();
        lb.set_output(Output::Buffer(&mut output));

        let input = String::from("\n\n test post text\n\n");
        let reader = io::Cursor::new(&input);
        lb.process(reader);
        assert_eq!("\n\n-------\n test post text\n\n", &output);
    }

    #[test]
    fn linurgy_run() {
        let input = String::from("test\nlines\n");
        let mut output = String::new();
        let mut lb = LinurgyBuilder::new();
        lb.set_input(Input::Buffer(&input));
        lb.set_output(Output::Buffer(&mut output));
        lb.run();
        assert_eq!("test\nlines\n", &output);
    }
}
