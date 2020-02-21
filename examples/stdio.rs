use linurgy::*;

fn main() -> Result<(), std::io::Error> {
    LinurgyBuilder::new()
        .set_new_text("\n")
        .set_edit_type(EditType::Replace)
        .run()?;

    Ok(())
}
