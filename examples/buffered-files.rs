use std::{
    env::temp_dir,
    fs,
    io::{BufReader, Result, Write},
    path::Path,
};

const INPUT_PATH: &str = "linurgy-input.txt";
const OUTPUT_PATH: &str = "linurgy-output.txt";

/// Append underscores to a double newline using a buffered
/// input and output file.
fn main() -> Result<()> {
    // appends "___" after every 2 newlines
    let editor = linurgy::factory::appender("___\n", 2);

    // temp file paths
    let input_path = temp_dir().join(INPUT_PATH);
    let output_path = temp_dir().join(OUTPUT_PATH);

    // setup the input file
    let input = "example line\n\nanother line\n";
    create_input_file(input, &input_path)?;

    // open the files
    let input_file = fs::File::open(&input_path)?;
    let mut output_file = fs::File::create(&output_path)?;

    // wrap the input in a buffered reader
    let mut input_buf = BufReader::new(input_file);

    // `edit_buffered` returns an io::Result
    editor.edit_buffered(&mut input_buf, &mut output_file)?;

    // check the output
    let expected = "example line\n\n___\nanother line\n";
    assert_and_print(input, expected, &output_path)?;

    // remove the files
    remove_files(input_buf, output_file, &input_path, &output_path)?;

    Ok(())
}

fn create_input_file(input: &str, path: &Path) -> Result<()> {
    let mut input_file = fs::File::create(path)?;
    input_file.write_all(input.as_bytes())?;
    Ok(())
}

fn assert_and_print(input: &str, expected: &str, path: &Path) -> Result<()> {
    let output = fs::read_to_string(path)?;
    assert_eq!(expected, output);
    println!("input:\n{}\noutput:\n{}", input, output);
    Ok(())
}

fn remove_files(
    input_buf: BufReader<fs::File>,
    output_file: fs::File,
    input_path: &Path,
    output_path: &Path,
) -> Result<()> {
    // close the files
    drop(input_buf);
    output_file.sync_all()?;
    drop(output_file);

    // remove the files
    fs::remove_file(input_path)?;
    fs::remove_file(output_path)?;

    Ok(())
}
