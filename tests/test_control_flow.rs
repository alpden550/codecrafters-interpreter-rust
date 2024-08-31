use crate::fixtures::create_temp_file_with_content;
use std::io;
use std::process::Command;

mod fixtures;

#[test]
fn test_if_condition_error_without_paren() -> io::Result<()> {
    let content = r#"
    if true == !nil {
        print "Not none";
    }
    "#;
    let tmp_file = create_temp_file_with_content(content.trim())?;
    let filepath = tmp_file.path();

    let output = Command::new("bash")
        .arg("your_program.sh")
        .arg(filepath)
        .output()?;

    let expected = r#"
[line 1] Expect '(' after 'if'.
[line 3] Expect expression.
    "#;

    assert_ne!(output.status.success(), true);
    assert_eq!(
        String::from_utf8_lossy(&output.stderr).trim(),
        expected.trim()
    );

    Ok(())
}

#[test]
fn test_if_condition_error_without_brace() -> io::Result<()> {
    let content = r#"
    if (true == !nil) 
        print "Not none";
    "#;
    let tmp_file = create_temp_file_with_content(content.trim())?;
    let filepath = tmp_file.path();

    let output = Command::new("bash")
        .arg("your_program.sh")
        .arg(filepath)
        .output()?;

    let expected = r#"
[line 2] Expect { before if body
    "#;

    assert_ne!(output.status.success(), true);
    assert_eq!(
        String::from_utf8_lossy(&output.stderr).trim(),
        expected.trim()
    );

    Ok(())
}

#[test]
fn test_if_condition_success() -> io::Result<()> {
    let content = r#"
    if (true == !nil) {
        print "Not none";
    }
    "#;
    let tmp_file = create_temp_file_with_content(content.trim())?;
    let filepath = tmp_file.path();

    let output = Command::new("bash")
        .arg("your_program.sh")
        .arg(filepath)
        .output()?;

    let expected = r#"
Not none
    "#;

    assert_eq!(output.status.success(), true);
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        expected.trim()
    );

    Ok(())
}

#[test]
fn test_logical_operators_success() -> io::Result<()> {
    let content = r#"
    print "hi" or 2;
    print nil or "yes";
    print 1 and 0;
    print 1 and 1;
    "#;
    let tmp_file = create_temp_file_with_content(content.trim())?;
    let filepath = tmp_file.path();

    let output = Command::new("bash")
        .arg("your_program.sh")
        .arg(filepath)
        .output()?;
    let expected = r#"
hi
yes
0
1
    "#;

    assert_eq!(output.status.success(), true);
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        expected.trim()
    );

    Ok(())
}

#[test]
fn test_while_loop_success() -> io::Result<()> {
    let content = r#"
    var condition = 0;
    while (condition <= 5) {
        print condition;
        condition = condition + 1;
    }
    "#;
    let tmp_file = create_temp_file_with_content(content.trim())?;
    let filepath = tmp_file.path();

    let output = Command::new("bash")
        .arg("your_program.sh")
        .arg(filepath)
        .output()?;
    let expected = r#"
0
1
2
3
4
5
    "#;

    assert_eq!(output.status.success(), true);
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        expected.trim()
    );

    Ok(())
}

#[test]
fn test_for_loop_success() -> io::Result<()> {
    let content = r#"
    for (var i = 0; i < 3; i = i + 1) {
        print i;
    }
    "#;
    let tmp_file = create_temp_file_with_content(content.trim())?;
    let filepath = tmp_file.path();

    let output = Command::new("bash")
        .arg("your_program.sh")
        .arg(filepath)
        .output()?;
    let expected = r#"
0
1
2
    "#;

    assert_eq!(output.status.success(), true);
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        expected.trim()
    );

    Ok(())
}

#[test]
fn test_for_loop_error() -> io::Result<()> {
    let content = r#"
    for (var i = 0; i < 3; i = i + 1) {
        print i;
    }
    print i;
    "#;
    let tmp_file = create_temp_file_with_content(content.trim())?;
    let filepath = tmp_file.path();

    let output = Command::new("bash")
        .arg("your_program.sh")
        .arg(filepath)
        .output()?;
    let expected = r#"
[line 4] Undefined variable 'i'.
    "#;

    assert_ne!(output.status.success(), true);
    assert_eq!(
        String::from_utf8_lossy(&output.stderr).trim(),
        expected.trim()
    );

    Ok(())
}
