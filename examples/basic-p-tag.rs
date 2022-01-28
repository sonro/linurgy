/// Add html style <p> tags around paragraphs
fn main() {
    // when double newlines are encountered, close the <p> tag then open a new one
    // keep the double newline for readablity
    let editor = linurgy::factory::replacer("</p>\n\n<p>", 2);

    let input = r#"Incididunt sit aute laboris veniam anim non tempor. Cillum
laborum id minim tempor quis magna consequat labore. Quis veniam amet cupidatat
Incididunt labore Lorem eu mollit laborum elit.

Commodo sint fugiat in in nisi aliquip qui irure aliqua aliqua esse aute
voluptate. Ad reprehenderit quis fugiat deserunt in est proident laboris enim.
Laborum fugiat deserunt ut consectetur.

Mollit laboris quis mollit veniam amet occaecat nulla id nulla. Duis irure
fugiat consectetur ipsum culpa. Et eiusmod mollit elit anim."#;

    let output = editor.edit(input);

    let expected = r#"Incididunt sit aute laboris veniam anim non tempor. Cillum
laborum id minim tempor quis magna consequat labore. Quis veniam amet cupidatat
Incididunt labore Lorem eu mollit laborum elit.</p>

<p>Commodo sint fugiat in in nisi aliquip qui irure aliqua aliqua esse aute
voluptate. Ad reprehenderit quis fugiat deserunt in est proident laboris enim.
Laborum fugiat deserunt ut consectetur.</p>

<p>Mollit laboris quis mollit veniam amet occaecat nulla id nulla. Duis irure
fugiat consectetur ipsum culpa. Et eiusmod mollit elit anim."#;

    assert_eq!(expected, output);

    // we have to add the first and last tags manually
    println!("input:\n{}\n\noutput:\n<p>{}</p>\n", input, output);
}
