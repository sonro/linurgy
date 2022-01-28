use linurgy::{Input, LinurgyBuilder, Output};

fn main() -> Result<(), std::io::Error> {
    let input = String::from("Some sample text\n\n\nResult\n");
    let mut buffer = String::new();

    LinurgyBuilder::new()
        .set_input(Input::Buffer(&input))
        .set_output(Output::Buffer(&mut buffer))
        .run()?;

    print!("{}", buffer);

    Ok(())
}
