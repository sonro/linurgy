/// Double lines
fn main() {
    // create an editor that adds an empty line after each line
    let editor = linurgy::factory::appender("\n", 1);

    // note the single line breaks
    let input = "example line\nanother line\n";

    let output = editor.edit(input);

    // note the double line breaks
    let expected = "example line\n\nanother line\n\n";

    assert_eq!(expected, output);

    print!("input:\n{}\noutput:\n{}", input, output);
}
