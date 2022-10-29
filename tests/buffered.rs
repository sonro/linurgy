// use std::{
//     env::temp_dir,
//     fs,
//     io::{BufReader, Result, Write},
//     path::Path,
// };

// use linurgy::factory;

// #[test]
// fn extra_line() {
//     let input = "foo\nbar\nbaz\n";

//     let expected = "foo\n\nbar\n\nbaz\n\n";

//     let mut input_buf = BufReader::new(input.as_bytes());
//     let mut output_buf = Vec::<u8>::with_capacity(input.len());

//     let res = factory::appender("\n", 1).edit_buffered(&mut input_buf, &mut output_buf);

//     assert!(res.is_ok());

//     let actual = String::from_utf8_lossy(&output_buf);

//     assert_eq!(expected, actual);
// }

// #[test]
// fn extra_line_crlf() {
//     let input = "foo\r\nbar\r\nbaz\r\n";

//     let expected = "foo\r\n\r\nbar\r\n\r\nbaz\r\n\r\n";

//     let mut input_buf = BufReader::new(input.as_bytes());
//     let mut output_buf = Vec::<u8>::with_capacity(input.len());

//     let res = factory::appender_crlf("\r\n", 1).edit_buffered(&mut input_buf, &mut output_buf);

//     assert!(res.is_ok());

//     let actual = String::from_utf8_lossy(&output_buf);

//     assert_eq!(expected, actual);
// }

// #[test]
// fn files() -> Result<()> {
//     let input = "foo\n\nbar\n\nbaz\n";

//     let expected = "foo--\n\nbar--\n\nbaz\n";

//     let input_path = temp_dir().join("linurgy-input.txt");
//     let output_path = temp_dir().join("linurgy-output.txt");

//     create_input_file(input, &input_path)?;

//     let mut input = BufReader::new(fs::File::open(&input_path)?);
//     let mut output = fs::File::create(&output_path)?;

//     factory::inserter("--", 2).edit_buffered(&mut input, &mut output)?;

//     let actual = fs::read_to_string(&output_path)?;

//     assert_eq!(expected, actual);

//     // drop so we can remove the files
//     drop(input);
//     drop(output);
//     drop(actual);

//     fs::remove_file(&input_path)?;
//     fs::remove_file(&output_path)?;

//     Ok(())
// }

// fn create_input_file(input: &str, path: &Path) -> Result<()> {
//     let mut input_file = fs::File::create(path)?;
//     input_file.write_all(input.as_bytes())?;
//     Ok(())
// }
