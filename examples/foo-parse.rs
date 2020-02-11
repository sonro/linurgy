use linurgy::*;

fn main() {
    let input = String::from("\nfoo\n\nbar\n\nbaz\n");
    let mut buffer = String::new();

    LinurgyBuilder::new()
        .set_input(Input::Buffer(&input))
        .set_newline_trigger(1)
        .set_new_text("FOO")
        .set_edit_type(EditType::Replace)
        .set_output(Output::Buffer(&mut buffer))
        .run();

    println!("{}", buffer);
}
