use air_lang::interpreter::Interpreter;

#[test]
fn it_executes_a_simple_script() {
    let interpreter = Interpreter::new();

    let sources = [
        ("+1", 1),
        ("-1", -1),
        ("1 + 1", 2),
        ("2 - 1", 1),
        ("1 - 1", 0),
        ("2 + 3 - 1", 4),
        ("1 + (3 - 2) + 4 - 6", 0),
        ("1 + 1 -     (2 + 1   + (3 - 2)) + 12", 10),
    ];

    for (source, expected) in sources {
        let actual = interpreter.execute(source);
        assert!(actual.is_ok());
        assert_eq!(expected, actual.unwrap(), "Failed with source expression: `{}`", source);
    }
}