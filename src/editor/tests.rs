use super::NewlineType;

pub struct EditTest {
    pub name: &'static str,
    pub expected: &'static str,
    pub input: &'static str,
    pub trigger: u8,
    pub replace: &'static str,
    pub newline: NewlineType,
}

pub const EDIT_TESTS: &[EditTest] = &[
    EditTest {
        name: "leading newline preserved",
        expected: "\nfoo\nbar\nbaz\n",
        input: "\nfoo\nbar\nbaz\n",
        trigger: 2,
        replace: "",
        newline: NewlineType::Lf,
    },
    EditTest {
        name: "leading newline preserved: crlf",
        expected: "\r\nfoo\r\nbar\r\nbaz\r\n",
        input: "\r\nfoo\r\nbar\r\nbaz\r\n",
        trigger: 2,
        replace: "",
        newline: NewlineType::Crlf,
    },
    EditTest {
        name: "no trailing newline preserved",
        expected: "foo\nbar\nbaz",
        input: "foo\nbar\nbaz",
        trigger: 2,
        replace: "",
        newline: NewlineType::Lf,
    },
    EditTest {
        name: "no trailing newline preserved: crlf",
        expected: "foo\r\nbar\r\nbaz",
        input: "foo\r\nbar\r\nbaz",
        trigger: 2,
        replace: "",
        newline: NewlineType::Crlf,
    },
    EditTest {
        name: "insert dash every line",
        expected: "foo-\nbar-\nbaz-\n",
        input: "foo\nbar\nbaz\n",
        trigger: 1,
        replace: "-\n",
        newline: NewlineType::Lf,
    },
    EditTest {
        name: "insert dash every line: crlf",
        expected: "foo-\r\nbar-\r\nbaz-\r\n",
        input: "foo\r\nbar\r\nbaz\r\n",
        trigger: 1,
        replace: "-\r\n",
        newline: NewlineType::Crlf,
    },
    EditTest {
        name: "append dash every line",
        expected: "foo\n-bar\n-baz\n-",
        input: "foo\nbar\nbaz\n",
        trigger: 1,
        replace: "\n-",
        newline: NewlineType::Lf,
    },
    EditTest {
        name: "append dash every line: crlf",
        expected: "foo\r\n-bar\r\n-baz\r\n-",
        input: "foo\r\nbar\r\nbaz\r\n",
        trigger: 1,
        replace: "\r\n-",
        newline: NewlineType::Crlf,
    },
    EditTest {
        name: "replace with dash every line",
        expected: "foo-bar-baz",
        input: "foo\nbar\nbaz",
        trigger: 1,
        replace: "-",
        newline: NewlineType::Lf,
    },
    EditTest {
        name: "replace with dash every line: crlf",
        expected: "foo-bar-baz",
        input: "foo\r\nbar\r\nbaz",
        trigger: 1,
        replace: "-",
        newline: NewlineType::Crlf,
    },
    EditTest {
        name: "append extra line",
        expected: "foo\n\nbar\n\nbaz\n\n",
        input: "foo\nbar\nbaz\n",
        trigger: 1,
        replace: "\n\n",
        newline: NewlineType::Lf,
    },
    EditTest {
        name: "append extra line: crlf",
        expected: "foo\r\n\r\nbar\r\n\r\nbaz\r\n\r\n",
        input: "foo\r\nbar\r\nbaz\r\n",
        trigger: 1,
        replace: "\r\n\r\n",
        newline: NewlineType::Crlf,
    },
    EditTest {
        name: "remove extra line",
        expected: "foo\nbar\nbaz\n",
        input: "foo\n\nbar\n\nbaz\n\n",
        trigger: 2,
        replace: "\n",
        newline: NewlineType::Lf,
    },
    EditTest {
        name: "remove extra line: crlf",
        expected: "foo\r\nbar\r\nbaz\r\n",
        input: "foo\r\n\r\nbar\r\n\r\nbaz\r\n\r\n",
        trigger: 2,
        replace: "\r\n",
        newline: NewlineType::Crlf,
    },
    EditTest {
        name: "zero trigger does nothing",
        expected: "foo\nbar\n\nbaz\n\n\n",
        input: "foo\nbar\n\nbaz\n\n\n",
        trigger: 0,
        replace: "should not be used",
        newline: NewlineType::Lf,
    },
    EditTest {
        name: "zero trigger does nothing: crlf",
        expected: "foo\r\nbar\r\n\r\nbaz\r\n\r\n\r\n",
        input: "foo\r\nbar\r\n\r\nbaz\r\n\r\n\r\n",
        trigger: 0,
        replace: "should not be used",
        newline: NewlineType::Crlf,
    },
    EditTest {
        name: "insert dash every 5 lines",
        expected: "foo-\n\n\n\n\n-\n\n\n\n\n",
        input: "foo\n\n\n\n\n\n\n\n\n\n",
        trigger: 5,
        replace: "-\n\n\n\n\n",
        newline: NewlineType::Lf,
    },
    EditTest {
        name: "insert dash every 4 lines: crlf",
        expected: "foo-\r\n\r\n\r\n\r\n-\r\n\r\n\r\n\r\n",
        input: "foo\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n",
        trigger: 4,
        replace: "-\r\n\r\n\r\n\r\n",
        newline: NewlineType::Crlf,
    },
];
