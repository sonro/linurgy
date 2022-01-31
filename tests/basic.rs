use linurgy::factory;

#[test]
fn extra_line() {
    let input = "foo\nbar\nbaz\n";

    let expected = "foo\n\nbar\n\nbaz\n\n";

    let actual = factory::appender("\n", 1).edit(input);

    assert_eq!(expected, actual);
}

#[test]
fn extra_line_crlf() {
    let input = "foo\r\nbar\r\nbaz\r\n";

    let expected = "foo\r\n\r\nbar\r\n\r\nbaz\r\n\r\n";

    let actual = factory::appender_crlf("\r\n", 1).edit(input);

    assert_eq!(expected, actual);
}

#[test]
fn insert_dashes_every_two_newlines() {
    let input = "foo\n\nbar\n\nbaz\n";

    let expected = "foo--\n\nbar--\n\nbaz\n";

    let actual = factory::inserter("--", 2).edit(input);

    assert_eq!(expected, actual);
}

#[test]
fn insert_dashes_every_two_newlines_crlf() {
    let input = "foo\r\n\r\nbar\r\n\r\nbaz\r\n";

    let expected = "foo--\r\n\r\nbar--\r\n\r\nbaz\r\n";

    let actual = factory::inserter_crlf("--", 2).edit(input);

    assert_eq!(expected, actual);
}

#[test]
fn reuse_replacer() {
    let editor = factory::replacer("-", 1);

    let input = "foo\nbar\nbaz";

    let expected = "foo-bar-baz";

    let actual = editor.edit(input);

    assert_eq!(expected, actual);

    let input = "tooth\n\nfairy";

    let expected = "tooth--fairy";

    let actual = editor.edit(input);

    assert_eq!(expected, actual);
}
