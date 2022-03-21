use air_lang::interpreter::Interpreter;

#[test]
fn it_executes_some_simple_scripts() {
    let interpreter = Interpreter::new();

    let sources = [
        ("+1", 1),
        ("-1", -1),
        ("1 + 1", 2),
        ("2 - 1", 1),
        ("1 - 1", 0),
        ("-1 + 1", 0),
        ("-1 - 1", -2),
        ("2 + 3 - 1", 4),
        ("1 + (3 - 2) + 4 - 6", 0),
        ("2 * 2", 4),
        ("2 * -3", -6),
        ("2^3 + 0", 8),
        ("-2^3 + 2", -6),
        ("1 + 1 -     (2 + 1   + (3 - 2)) + 12", 10),
        ("2 + 2 ^ 3", 10),
        ("2 + 2^3", 10),
        ("2 + (2^3 + 1)", 11)
    ];

    for (source, expected) in sources {
        let actual = interpreter.execute(source);
        assert!(actual.is_ok());
        assert_eq!(expected, actual.unwrap(), "Failed with source expression: `{}`", source);
    }
}