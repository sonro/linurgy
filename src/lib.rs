#![doc(html_root_url = "https://docs.rs/linurgy/0.4.0")]
//! `linurgy` provides an interface for manipulating multiple newlines in text.
//! Interaction with this library happens through
//! [`LinurgyBuilder`](struct.LinurgyBuilder.html).
//!
//! # Examples
//!
//! Read stdin and for each empty line, append an extra line to stdout.
//! ```rust
//! # use std::error::Error;
//! # use linurgy::LinurgyBuilder;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! LinurgyBuilder::new()
//!     .set_newline_trigger(2)
//!     .set_new_text("\n")
//!     .run()?;
//! #
//! # Ok(())
//! # }
//! ```
//!
//! Read from one buffer, remove all empty lines, and output to another buffer.
//! ```rust
//! # use std::error::Error;
//! # use linurgy::{LinurgyBuilder, Input, Output, EditType};
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let input = String::from("Remove\n\nEvery\n\nEmpty\n\nLine\n");
//! let mut output = String::new();
//!
//! LinurgyBuilder::new()
//!     .set_input(Input::Buffer(&input))
//!     .set_output(Output::Buffer(&mut output))
//!     .set_newline_trigger(2)
//!     .set_edit_type(EditType::Replace)
//!     .set_new_text("\n")
//!     .run();
//!
//! assert_eq!("Remove\nEvery\nEmpty\nLine\n", &output);
//! #
//! # Ok(())
//! # }
//! ```

pub mod builder;
pub mod editor;

use std::fs;
use std::io::{self, Write};

/// Type of input stream to edit
#[derive(PartialEq, Debug)]
pub enum Input<'a> {
    /// Basic line by line read from stdin
    StdIn,

    /// Read from a given filename
    File(&'a str),

    /// Read from a string
    Buffer(&'a str),
}

/// Type of output stream to write edits to
#[derive(PartialEq, Debug)]
pub enum Output<'b> {
    /// Basic line by line output to stdout
    StdOut,

    /// Write to a given filename
    File(&'b str),

    /// Write to a given `String` buffer
    Buffer(&'b mut String),
}

/// Which action to implement when editing newlines
#[derive(PartialEq, Debug)]
pub enum EditType {
    /// New edits will appear after newlines
    Append,

    /// New edits will appear before newlines
    Insert,

    /// New edits will appear instead of newlines
    Replace,
}

/// Use this to prepare and execute linurgy editing on a stream.
///
/// A linurgy consists of an [`Input`](enum.Input.html), which will be read
/// line by line, edited by user defined rules, and then streamed into an
/// [`Output`](enum.Output.html).
#[derive(Debug)]
pub struct LinurgyBuilder<'a, 'b, 'c> {
    input: Input<'a>,
    output: Output<'b>,
    newline_count_trigger: u8,
    new_text: &'c str,
    edit_type: EditType,
    buffer: String,
    file: Option<fs::File>,
}

impl Default for LinurgyBuilder<'_, '_, '_> {
    fn default() -> Self {
        LinurgyBuilder {
            input: Input::StdIn,
            output: Output::StdOut,
            newline_count_trigger: 2,
            new_text: "-------\n",
            edit_type: EditType::Append,
            buffer: String::new(),
            file: None,
        }
    }
}

impl<'a, 'b, 'c> LinurgyBuilder<'a, 'b, 'c> {
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
    /// let mut linurgy = LinurgyBuilder::new();
    ///
    /// linurgy.set_input(Input::File("filename.txt"));
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
    /// let mut linurgy = LinurgyBuilder::new();
    ///
    /// linurgy.set_output(Output::File("filename.txt"));
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
        self.newline_count_trigger = count;
        self
    }

    /// Set the text that will be used when the newline trigger is reached.
    ///
    /// # Example
    /// Add a line of dots after every empty line
    /// ```rust
    /// # use linurgy::{LinurgyBuilder};
    /// let mut linurgy = LinurgyBuilder::new();
    /// let new_text = format!("{}\n", ". ".repeat(25));
    ///
    /// linurgy.set_newline_trigger(1);
    /// linurgy.set_new_text(&new_text);
    /// ```
    pub fn set_new_text(&mut self, new_text: &'c str) -> &mut Self {
        self.new_text = new_text;
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
        self.edit_type = edit_type;
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
    /// # Examples
    /// Execute default behaviour and add dashes to
    /// double newlines from `stdin`
    /// ```rust
    /// # use std::error::Error;
    /// # use linurgy::LinurgyBuilder;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// LinurgyBuilder::new().run()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// If this function encounters any form of I/O or other error, an error
    /// variant will be returned.
    pub fn run(&mut self) -> Result<(), io::Error> {
        if let Output::File(path) = &self.output {
            self.file = Some(fs::File::create(path)?)
        }

        match self.input {
            Input::StdIn => {
                let stdin = io::stdin();
                let reader = stdin.lock();
                self.process(reader)?;
            }
            Input::File(name) => {
                let file = fs::File::open(name)?;
                let reader = io::BufReader::new(file);
                self.process(reader)?;
            }
            Input::Buffer(buffer) => {
                let reader = io::Cursor::new(buffer);
                self.process(reader)?;
            }
        }

        Ok(())
    }

    fn add_newlines_to_buffer(&mut self, count: u8) {
        for _ in 0..count {
            self.buffer += "\n";
        }
    }

    fn process(&mut self, reader: impl io::BufRead) -> Result<(), io::Error> {
        let mut newlines = 0;
        let mut rollon = false;

        for line in reader.lines() {
            let line = line?;
            if !line.is_empty() && line.chars().any(|c| !c.is_whitespace()) {
                if rollon {
                    self.add_newlines_to_buffer(1);
                }
                newlines = 1;
                self.buffer += &line;
            } else {
                newlines += 1;
            }

            if newlines == self.newline_count_trigger {
                match self.edit_type {
                    EditType::Append => {
                        self.add_newlines_to_buffer(newlines);
                        self.buffer += self.new_text;
                    }
                    EditType::Insert => {
                        self.buffer += self.new_text;
                        self.add_newlines_to_buffer(newlines);
                    }
                    EditType::Replace => self.buffer += self.new_text,
                }
                rollon = false;
                newlines = 0;
            } else {
                rollon = true;
            }

            self.write()?;
        }

        if rollon {
            self.add_newlines_to_buffer(1);
            self.write()?;
        }

        Ok(())
    }

    fn write(&mut self) -> Result<(), io::Error> {
        match self.output {
            Output::StdOut => print!("{}", self.buffer),
            Output::File(_) => {
                if let Some(file) = &mut self.file {
                    file.write_all(self.buffer.as_bytes())?
                } else {
                    return Err(io::Error::from(io::ErrorKind::NotFound));
                }
            }
            Output::Buffer(ref mut buffer) => buffer.push_str(&self.buffer),
        }
        self.buffer.clear();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXPECT_MSG: &str = "should never error";

    #[test]
    fn default_linurgy_builder() {
        let lb = LinurgyBuilder::new();
        let default = LinurgyBuilder {
            input: Input::StdIn,
            output: Output::StdOut,
            newline_count_trigger: 2,
            new_text: "-------\n",
            edit_type: EditType::Append,
            buffer: String::new(),
            file: None,
        };
        assert_eq!(default.input, lb.input);
        assert_eq!(default.output, lb.output);
        assert_eq!(default.newline_count_trigger, lb.newline_count_trigger);
        assert_eq!(default.new_text, lb.new_text);
        assert_eq!(default.edit_type, lb.edit_type);
        assert_eq!(default.buffer, lb.buffer);
        assert_eq!(default.file.is_none(), lb.file.is_none());
    }

    #[test]
    fn linurgy_set_input() {
        let buffer = String::from("Test builder");
        let mut lb = LinurgyBuilder::new();

        lb.set_input(Input::Buffer(&buffer));
        match lb.input {
            Input::Buffer(text) => assert_eq!(&buffer, text),
            _ => panic!("Correct type not implemented"),
        }

        lb.set_input(Input::File("filename"));
        match lb.input {
            Input::File(text) => assert_eq!("filename", text),
            _ => panic!("Correct type not implemented"),
        }

        lb.set_input(Input::StdIn);
        assert_eq!(Input::StdIn, lb.input);
    }

    #[test]
    fn linurgy_set_output() {
        let mut buffer = String::from("Test builder");
        let mut buffer2 = String::from("Test builder");
        let mut lb = LinurgyBuilder::new();

        lb.set_output(Output::Buffer(&mut buffer));
        match lb.output {
            Output::Buffer(ref text) => assert_eq!(&&mut buffer2, text),
            _ => panic!("Correct type not implemented"),
        }

        lb.set_output(Output::File("filename"));
        match lb.output {
            Output::File(text) => assert_eq!("filename", text),
            _ => panic!("Correct type not implemented"),
        }

        lb.set_output(Output::StdOut);
        assert_eq!(Output::StdOut, lb.output);
    }

    #[test]
    fn linurgy_set_newline_trigger() {
        let mut lb = LinurgyBuilder::new();
        lb.set_newline_trigger(5);
        assert_eq!(5, lb.newline_count_trigger);
    }

    #[test]
    fn linurgy_set_new_text() {
        let mut lb = LinurgyBuilder::new();
        lb.set_new_text("cheese");
        assert_eq!("cheese", lb.new_text);
    }

    #[test]
    fn linurgy_set_edit_type() {
        let mut lb = LinurgyBuilder::new();
        lb.set_edit_type(EditType::Insert);
        assert_eq!(EditType::Insert, lb.edit_type);
    }

    fn get_testable_linurgy_builder<'a, 'b, 'c>(
        output: &'b mut String,
    ) -> LinurgyBuilder<'a, 'b, 'c> {
        output.clear();
        let mut lb = LinurgyBuilder::new();
        lb.set_output(Output::Buffer(output));
        lb
    }

    #[test]
    fn linurgy_write() {
        let mut output = String::new();
        let mut lb = get_testable_linurgy_builder(&mut output);

        lb.buffer = "testline\n".to_owned();
        lb.write().expect(EXPECT_MSG);
        assert_eq!("testline\n", output);

        let mut lb = get_testable_linurgy_builder(&mut output);

        lb.buffer = "testline\n".to_owned();
        lb.write().expect(EXPECT_MSG);
        lb.buffer = "testline\n".to_owned();
        lb.write().expect(EXPECT_MSG);
        assert_eq!("testline\ntestline\n", output);
    }

    #[test]
    fn linurgy_process() {
        fn assert_expected_output_from_input(expected: &str, input: &str) {
            let mut output = String::new();
            let mut lb = get_testable_linurgy_builder(&mut output);
            let input = String::from(input);
            let reader = io::Cursor::new(&input);
            lb.process(reader).expect(EXPECT_MSG);
            assert_eq!(expected, &output);
        }

        assert_expected_output_from_input("test\nlines\n", "test\nlines\n");
        assert_expected_output_from_input("\n\n-------\n", "\n\n");
        assert_expected_output_from_input(
            "\n\n-------\n test post text\n\n-------\n",
            "\n\n test post text\n\n",
        );
    }

    #[test]
    fn linurgy_run() {
        let input = String::from("test\nlines\n");
        let mut output = String::new();
        let mut lb = LinurgyBuilder::new();
        lb.set_input(Input::Buffer(&input));
        lb.set_output(Output::Buffer(&mut output));
        lb.run().expect(EXPECT_MSG);
        assert_eq!("test\nlines\n", &output);
    }
}
