use crate::fixtures::interpret_temp_file_with_content;
use std::io;

mod fixtures;

#[test]
fn test_print_func() -> io::Result<()> {
    let content = r#"
        var n = "Hello, tests!";
        print n;
        "#;

    let output = interpret_temp_file_with_content(content)?;
    let expected_stdout = "Hello, tests!\n";

    assert_eq!(output.status.success(), true);
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected_stdout);
    assert_eq!(output.stderr.is_empty(), true);

    Ok(())
}
