use std::io::{stdin, stdout, BufReader, Result};

/// Buffered edit over stdin and stdout.
///
/// This will add underscores after every line and doesn't need to wait for
/// the user to end the input stream. Useful for watching files such as logs.
fn main() -> Result<()> {
    // appeneds "___\n" after every line
    let editor = linurgy::factory::appender("___\n", 1);

    // create a buffered reader over stdin
    let mut input = BufReader::new(stdin());

    // `edit_buffered` returns an io::Result
    // in a simple program like this, we can pass stdout directly
    editor.edit_buffered(&mut input, &mut stdout())?;

    Ok(())
}
