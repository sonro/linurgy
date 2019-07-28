pub enum Input {
    StdIn,
    File(String),
    Buffer(String),
}

pub enum Output<'b> {
    StdOut,
    File(String),
    Buffer(String),
}

pub struct Editor {
    newline_count_trigger: u8,
    new_text: String,
    current_count: u8,
    buffer: String
}

pub struct LinurgyBuilder {
    input:  Input,
    output: Output,
    editor: Editor,
}

