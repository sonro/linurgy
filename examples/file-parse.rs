use linurgy::*;
use std::fs;

fn main() {
    let data = String::from("this\n\nis\n\nfile\n");
    let tmpinfilename = "/tmp/linurgy-test-input-file";
    let tmpoutfilename = "/tmp/linurgy-test-output-file";
    fs::write(tmpinfilename, &data).expect("Write to file");
    println!("Initial file:\n{}", &data);

    LinurgyBuilder::new()
        .set_input(Input::File(tmpinfilename))
        .set_newline_trigger(2)
        .set_new_text("\n_-_-_-_-_\n")
        .set_edit_type(EditType::Replace)
        .set_output(Output::File(tmpoutfilename))
        .run();

    let result = fs::read_to_string(tmpoutfilename).expect("Read from file");

    println!("Resulting file:\n{}", result);
}
