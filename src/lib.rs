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

pub struct LinurgyBuilder {
    input:  Input,
    output: Output,
    editor: Editor,
}

