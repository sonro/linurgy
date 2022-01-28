/// Replace all newlines with a dash
fn main() {
    let editor = linurgy::factory::replacer("-", 1);

    let input = "example line\nanother line";

    let output = editor.edit(input);

    let expected = "example line-another line";

    assert_eq!(expected, output);

    println!("input:\n{}\noutput:\n{}", input, output);
}
