use linurgy::*;

fn main() {
    LinurgyBuilder::new()
        .set_new_text("\n")
        .set_edit_type(EditType::Replace)
        .run();
}
