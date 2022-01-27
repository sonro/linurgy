use super::NewlineType;

pub struct EditTest {
    pub name: &'static str,
    pub expected: &'static str,
    pub input: &'static str,
    pub newlines: u8,
    pub replace: &'static str,
    pub line_ending: NewlineType,
}

macro_rules! editor_tests {
    ($assert_fn:ident) => {
        #[test]
        fn leading_newline_preserved() {
            $assert_fn(EditTest {
                name: "leading newline preserved",
                expected: "\nfoo\nbar\nbaz\n",
                input: "\nfoo\nbar\nbaz\n",
                newlines: 2,
                replace: "",
                line_ending: NewlineType::Lf,
            });
        }

        #[test]
        fn leading_newline_preserved_crlf() {
            $assert_fn(EditTest {
                name: "leading newline preserved crlf",
                expected: "\r\nfoo\r\nbar\r\nbaz\r\n",
                input: "\r\nfoo\r\nbar\r\nbaz\r\n",
                newlines: 2,
                replace: "",
                line_ending: NewlineType::Crlf,
            });
        }

        #[test]
        fn no_trailing_newline_preserved() {
            $assert_fn(EditTest {
                name: "no trailing newline preserved",
                expected: "foo\nbar\nbaz",
                input: "foo\nbar\nbaz",
                newlines: 2,
                replace: "",
                line_ending: NewlineType::Lf,
            });
        }

        #[test]
        fn no_trailing_newline_preserved_crlf() {
            $assert_fn(EditTest {
                name: "no trailing newline preserved crlf",
                expected: "foo\r\nbar\r\nbaz",
                input: "foo\r\nbar\r\nbaz",
                newlines: 2,
                replace: "",
                line_ending: NewlineType::Crlf,
            });
        }

        #[test]
        fn insert_dash_every_line() {
            $assert_fn(EditTest {
                name: "insert dash every line",
                expected: "foo-\nbar-\nbaz-\n",
                input: "foo\nbar\nbaz\n",
                newlines: 1,
                replace: "-\n",
                line_ending: NewlineType::Lf,
            });
        }

        #[test]
        fn insert_dash_every_line_crlf() {
            $assert_fn(EditTest {
                name: "insert dash every line crlf",
                expected: "foo-\r\nbar-\r\nbaz-\r\n",
                input: "foo\r\nbar\r\nbaz\r\n",
                newlines: 1,
                replace: "-\r\n",
                line_ending: NewlineType::Crlf,
            });
        }

        #[test]
        fn append_dash_every_line() {
            $assert_fn(EditTest {
                name: "append dash every line",
                expected: "foo\n-bar\n-baz\n-",
                input: "foo\nbar\nbaz\n",
                newlines: 1,
                replace: "\n-",
                line_ending: NewlineType::Lf,
            });
        }

        #[test]
        fn append_dash_every_line_crlf() {
            $assert_fn(EditTest {
                name: "append dash every line crlf",
                expected: "foo\r\n-bar\r\n-baz\r\n-",
                input: "foo\r\nbar\r\nbaz\r\n",
                newlines: 1,
                replace: "\r\n-",
                line_ending: NewlineType::Crlf,
            });
        }

        #[test]
        fn replace_with_dash_every_line() {
            $assert_fn(EditTest {
                name: "replace with dash every line",
                expected: "foo-bar-baz",
                input: "foo\nbar\nbaz",
                newlines: 1,
                replace: "-",
                line_ending: NewlineType::Lf,
            });
        }

        #[test]
        fn replace_with_dash_every_line_crlf() {
            $assert_fn(EditTest {
                name: "replace with dash every line crlf",
                expected: "foo-bar-baz",
                input: "foo\r\nbar\r\nbaz",
                newlines: 1,
                replace: "-",
                line_ending: NewlineType::Crlf,
            });
        }

        #[test]
        fn append_extra_line() {
            $assert_fn(EditTest {
                name: "append extra line",
                expected: "foo\n\nbar\n\nbaz\n\n",
                input: "foo\nbar\nbaz\n",
                newlines: 1,
                replace: "\n\n",
                line_ending: NewlineType::Lf,
            });
        }

        #[test]
        fn append_extra_line_crlf() {
            $assert_fn(EditTest {
                name: "append extra line crlf",
                expected: "foo\r\n\r\nbar\r\n\r\nbaz\r\n\r\n",
                input: "foo\r\nbar\r\nbaz\r\n",
                newlines: 1,
                replace: "\r\n\r\n",
                line_ending: NewlineType::Crlf,
            });
        }

        #[test]
        fn remove_extra_line() {
            $assert_fn(EditTest {
                name: "remove extra line",
                expected: "foo\nbar\nbaz\n",
                input: "foo\n\nbar\n\nbaz\n\n",
                newlines: 2,
                replace: "\n",
                line_ending: NewlineType::Lf,
            });
        }

        #[test]
        fn remove_extra_line_crlf() {
            $assert_fn(EditTest {
                name: "remove extra line crlf",
                expected: "foo\r\nbar\r\nbaz\r\n",
                input: "foo\r\n\r\nbar\r\n\r\nbaz\r\n\r\n",
                newlines: 2,
                replace: "\r\n",
                line_ending: NewlineType::Crlf,
            });
        }

        #[test]
        fn zero_newlines_does_nothing() {
            $assert_fn(EditTest {
                name: "zero newlines does nothing",
                expected: "foo\nbar\n\nbaz\n\n\n",
                input: "foo\nbar\n\nbaz\n\n\n",
                newlines: 0,
                replace: "should not be used",
                line_ending: NewlineType::Lf,
            });
        }

        #[test]
        fn zero_newlines_does_nothing_crlf() {
            $assert_fn(EditTest {
                name: "zero newlines does nothing crlf",
                expected: "foo\r\nbar\r\n\r\nbaz\r\n\r\n\r\n",
                input: "foo\r\nbar\r\n\r\nbaz\r\n\r\n\r\n",
                newlines: 0,
                replace: "should not be used",
                line_ending: NewlineType::Crlf,
            });
        }

        #[test]
        fn insert_dash_every_5_lines() {
            $assert_fn(EditTest {
                name: "insert dash every 5 lines",
                expected: "foo-\n\n\n\n\n-\n\n\n\n\n",
                input: "foo\n\n\n\n\n\n\n\n\n\n",
                newlines: 5,
                replace: "-\n\n\n\n\n",
                line_ending: NewlineType::Lf,
            });
        }

        #[test]
        fn insert_dash_every_4_lines_crlf() {
            $assert_fn(EditTest {
                name: "insert dash every 4 lines crlf",
                expected: "foo-\r\n\r\n\r\n\r\n-\r\n\r\n\r\n\r\n",
                input: "foo\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n",
                newlines: 4,
                replace: "-\r\n\r\n\r\n\r\n",
                line_ending: NewlineType::Crlf,
            });
        }

        #[test]
        fn replace_dash_every_3_lines() {
            $assert_fn(EditTest {
                name: "replace dash every 3 lines",
                expected: "foo-bar-baz",
                input: "foo\n\n\nbar\n\n\nbaz",
                newlines: 3,
                replace: "-",
                line_ending: NewlineType::Lf,
            });
        }
    };
}

pub(super) use editor_tests;
