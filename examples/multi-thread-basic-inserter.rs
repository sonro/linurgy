use std::sync::Arc;

/// Multi-threaded insert dashes
fn main() {
    // insert dashes before every double newline
    let editor = linurgy::factory::inserter("---", 2);
    let editor = Arc::new(editor);

    for _ in 0..10 {
        let editor = Arc::clone(&editor);

        std::thread::spawn(move || {
            // note the final single newline character
            let input = "example line\n\nanother line\n";

            let output = editor.edit(input);

            // single newline is untouched
            let expected = "example line---\n\nanother line\n";

            assert_eq!(expected, output);
        });
    }
}
