use linurgy::{LinurgyBuilder, Input, Output};

fn main() {
    let input = String::from("Some sample text\n\n\nResult\n");
    let mut buffer = String::new();

    LinurgyBuilder::new()
        .set_input(Input::Buffer(&input))
        .set_output(Output::Buffer(&mut buffer))
        .run();

    print!("{}", buffer);
}
