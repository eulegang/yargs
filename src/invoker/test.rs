use super::*;

#[test]
fn new() {
    assert_eq!(
        Invoker::new("%", vec!["echo".to_string(), "%".to_string()]),
        Invoker {
            templates: vec![Template::Static("echo".to_string())],
            offsets: vec![1]
        },
        "basic 'echo %' case"
    );

    assert_eq!(
        Invoker::new("{}", vec!["echo".to_string(), "%".to_string()]),
        Invoker {
            templates: vec![
                Template::Static("echo".to_string()),
                Template::Static("%".to_string())
            ],
            offsets: vec![]
        },
        "'echo %' with '{{}}' pattern"
    );

    assert_eq!(
        Invoker::new("{}", vec!["echo".to_string(), "{}".to_string()]),
        Invoker {
            templates: vec![Template::Static("echo".to_string())],
            offsets: vec![1]
        },
        "'echo {{}}' with '{{}}' pattern"
    );
}

#[test]
fn previews() {
    let echo = Invoker::new("%", vec!["echo".to_string(), "%".to_string()]);

    assert_eq!(
        echo.preview(&["foo"]).as_strs(),
        vec!["echo", "foo"],
        "simple 1 to 1 pattern fill"
    );

    assert_eq!(
        echo.preview(&["foo", "bar"]).as_strs(),
        vec!["echo", "foo", "bar"],
        "simple overfill"
    );

    assert_eq!(
        echo.preview(&[]).as_strs(),
        vec!["echo"],
        "simple underfill"
    );

    let cmd = Invoker::new(
        "%",
        vec![
            "cat".to_string(),
            "%".to_string(),
            "-E".to_string(),
            "%".to_string(),
        ],
    );

    assert_eq!(
        cmd.preview(&["hello", "world"]).as_strs(),
        vec!["cat", "hello", "-E", "world"],
        "multi slot simple fill"
    );

    assert_eq!(
        cmd.preview(&["hello"]).as_strs(),
        vec!["cat", "hello", "-E"],
        "multi slot underfill"
    );

    assert_eq!(
        cmd.preview(&["hello", "world", "foobar"]).as_strs(),
        vec!["cat", "hello", "-E", "world", "foobar"],
        "multi slot overfill"
    );

    assert_eq!(
        cmd.preview(&["hello", "world"]).as_strs().capacity(),
        4,
        "multi slot simple fill capacity"
    );

    assert_eq!(
        cmd.preview(&["hello", "world", "foobar"])
            .as_strs()
            .capacity(),
        5,
        "multi slot overfill capacity"
    );

    // even though 3 elements should exist we want to
    // have a decent heuristic for initial capacity of
    // the result set.
    assert_eq!(
        cmd.preview(&["hello"]).as_strs().capacity(),
        4,
        "multi slot underfill capacity"
    );
}

#[test]
fn template_new() {
    assert_eq!(
        Template::new("%", "hello".to_string()),
        Template::Static("hello".to_string())
    );

    assert_eq!(
        Template::new("%", "of=%".to_string()),
        Template::Interp {
            offsets: vec![3],
            base: "of=".to_string(),
        }
    );

    assert_eq!(
        Template::new("%", "%=%".to_string()),
        Template::Interp {
            offsets: vec![0, 1],
            base: "=".to_string(),
        }
    );

    assert_eq!(
        Template::new("%", "%=%xyz".to_string()),
        Template::Interp {
            offsets: vec![0, 1],
            base: "=xyz".to_string(),
        }
    );
}

#[test]
fn template_apply() {
    let template = Template::new("%", "of=%".to_string());
    let params = vec![&"/dev/null"];

    assert_eq!(
        template.apply(&mut params.into_iter()),
        "of=/dev/null".to_string()
    );

    let template = Template::new("%", "%=%".to_string());
    let params = vec![&"if", &"/dev/null"];

    assert_eq!(
        template.apply(&mut params.into_iter()),
        "if=/dev/null".to_string()
    );

    let template = Template::new("%", "%=%".to_string());
    let params = vec![&"if"];

    assert_eq!(template.apply(&mut params.into_iter()), "if=".to_string());
}
