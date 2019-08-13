use std::fs;
use linurgy::*;

fn main() {
    let data = String::from("this\n\nis\n\nfile\n");
    let tmpinfilename = "/tmp/linurgy-test-input-file";
    let tmpoutfilename = "/tmp/linurgy-test-output-file";
    fs::write(tmpinfilename, &data).expect("Write to file");
    
    println!("Initial file:\n{}", &data);

    LinurgyBuilder::new()
        .set_input(Input::File(tmpinfilename))
        .set_newline_trigger(1)
        // edit will be similar to Replace as there is no newline in new_text
        .set_new_text("_-_-_-_-_")
        .set_edit_type(EditType::Insert)
        .set_output(Output::File(tmpoutfilename))
        .run();

    let result = fs::read_to_string(tmpoutfilename).expect("Read from file");

    println!("Resulting file:\n{}", result);
}
