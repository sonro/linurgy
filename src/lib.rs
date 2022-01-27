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

pub mod editor;
pub mod factory;
mod legacy;

pub use editor::{EditType, Editor, NewlineType};
pub use legacy::*;
