use crate::fixtures::interpret_temp_file_with_content;
use std::io;

mod fixtures;

#[test]
fn test_global_scopes() -> io::Result<()> {
    let content = r#"
        var a = "global a";
        var b = "global b";
        var c = "global c";
            {
                var a = "outer a";
                var b = "outer b";
                    {
                        var a = "inner a";
                        print a;
                        print b;
                        print c;
                    }
                print a;
                print b;
                print c;
            }
        print a;
        print b;
        print c;
    "#;
    let output = interpret_temp_file_with_content(content.trim())?;
    let expected_stdout = r#"
inner a
outer b
global c
outer a
outer b
global c
global a
global b
global c
    "#;

    assert_eq!(output.status.success(), true);
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        expected_stdout.trim()
    );
    assert_eq!(output.stderr.is_empty(), true);

    Ok(())
}

#[test]
fn test_error_after_local_scope() -> io::Result<()> {
    let content = r#"
    {
        var calculated = (1234 * (1456/ 44 + (1987 - 264)) / 34);
        var is_true = true == !nil;
        print calculated;
        print is_true;
        print (true == !"") == (!false);
    }
    print calculated;
    print is_true;
    "#;

    let output = interpret_temp_file_with_content(content.trim())?;
    let expected = r#"
[line 8] Undefined variable 'calculated'.
[line 9] Undefined variable 'is_true'.
    "#;

    assert_eq!(output.status.code().unwrap(), 70);
    assert_eq!(
        String::from_utf8_lossy(&output.stderr).trim(),
        expected.trim()
    );

    Ok(())
}
