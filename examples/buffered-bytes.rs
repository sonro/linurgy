use std::io::{BufReader, Result};

/// Replace all newlines with a dash using buffered edit
fn main() -> Result<()> {
    // Create an editor in the same way as the basic example
    // This replaces every newline with a dash
    let editor = linurgy::factory::replacer("-", 1);

    let input = "example line\nanother line";

    // wrap the input in a buffered reader
    let mut input_buf = BufReader::new(input.as_bytes());

    // create a buffer to hold the output
    let mut output_buf: Vec<u8> = Vec::new();

    // `edit_buffered` returns an io::Result
    editor.edit_buffered(&mut input_buf, &mut output_buf)?;

    // convert the output buffer to a string
    let output = String::from_utf8_lossy(&output_buf);

    let expected = "example line-another line";

    assert_eq!(expected, output);

    println!("input:\n{}\noutput:\n{}", input, output);

    Ok(())
}
