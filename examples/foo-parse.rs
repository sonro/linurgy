use linurgy::*;

fn main() {
    let input = String::from("\nfoo\n\nbar\n\nbaz\n");
    let mut buffer = String::new();

    LinurgyBuilder::new()
        .add_input(Input::Buffer(&input))
        .add_newline_trigger(1)
        .add_new_text(String::from("FOO"))
        .add_edit_type(EditType::Replace)
        .add_output(Output::Buffer(&mut buffer))
        .run();

    print!("{}", buffer);
}
