use crate::fixtures::interpret_temp_file_with_content;
use std::io;

mod fixtures;

#[test]
fn test_func_without_brace_error() -> io::Result<()> {
    let content = r#"
        fun add a, b) {
            print a + b;
        }
        "#;

    let output = interpret_temp_file_with_content(content)?;
    assert_eq!(output.status.success(), false);
    assert_eq!(output.stdout.is_empty(), true);
    assert_eq!(output.stderr.is_empty(), false);

    Ok(())
}

#[test]
fn test_func_without_parent_error() -> io::Result<()> {
    let content = r#"
        fun add(a, b) 
            print a + b;
        "#;

    let output = interpret_temp_file_with_content(content)?;
    assert_eq!(output.status.success(), false);
    assert_eq!(output.stdout.is_empty(), true);
    assert_eq!(output.stderr.is_empty(), false);

    Ok(())
}

#[test]
fn test_func_return_none_successful() -> io::Result<()> {
    let content = r#"
        fun add(a, b) {
            print a + b;
        }
        var res = add(1, 2);
        print res;
        "#;

    let output = interpret_temp_file_with_content(content.trim())?;

    assert_eq!(output.status.success(), true);
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim().contains("3"),
        true
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .contains("nil"),
        true
    );
    assert_eq!(output.stderr.is_empty(), true);

    Ok(())
}

#[test]
fn test_func_return_result_successful() -> io::Result<()> {
    let content = r#"
        fun add(a, b) {
            return a + b;
        }
        var res = add(1, 2);
        print res;
        "#;

    let output = interpret_temp_file_with_content(content.trim())?;

    assert_eq!(output.status.success(), true);
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim().contains("3"),
        true
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .contains("nil"),
        false
    );
    assert_eq!(output.stderr.is_empty(), true);

    Ok(())
}
