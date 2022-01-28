/// Remove extra newline
fn main() {
    // setting `newlines` to `2` will replace "\n\n" with "\n"
    let editor = linurgy::factory::replacer("\n", 2);

    // note the final single newline character
    let input = "example line\n\nanother line\n";

    let output = editor.edit(input);

    // single newline is untouched
    let expected = "example line\nanother line\n";

    assert_eq!(expected, output);

    println!("input:\n{}\noutput:\n{}", input, output);
}
