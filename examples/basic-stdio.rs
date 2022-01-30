use std::io::{stdin, Read, Result};

/// Edit stdin and output to stdout.
///
/// This will add a line of dashes after every line. This will only output
/// after the user has ended the input stream. If you want to watch a file,
/// or use the program more interactively, use `buffered_edit` instead.
fn main() -> Result<()> {
    // appeneds "---\n" after every line
    let editor = linurgy::factory::appender("---\n", 1);

    // input buffer
    let mut input = String::new();

    // read the input
    stdin().read_to_string(&mut input)?;

    // write the output
    print!("{}", editor.edit(&input));

    Ok(())
}
