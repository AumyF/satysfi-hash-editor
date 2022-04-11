use std::io::Read;

use yojson_rs;
use yojson_rs::value::Value;

#[derive(Debug)]
enum Command {
    Set(String, String),
    Delete(String),
}

impl Command {
    fn new(args: Vec<String>) -> Command {
        let command = &args[1];
        let key = args[2].clone();
        let value = args.get(3);

        match value {
            Some(value) if command == "set" => Command::Set(key, value.to_string()),
            None if command == "delete" => Command::Delete(key),
            _ => panic!("too few or many arguments"),
        }
    }
}

fn edit(input: &str, command: Command) -> Value {
    let mut parsed = yojson_rs::parser::parse(input).expect("failed to parse as yojson");

    let root_map = parsed.as_assoc_mut().expect("expected assoc");

    match command {
        Command::Set(key, value) => {
            let variant = {
                let mut map = yojson_rs::value::Assoc::new();
                map.insert("src".to_string(), Value::String(value));
                Value::Variant(("Single".to_string(), Some(Box::new(Value::Assoc(map)))))
            };

            root_map.insert(key, variant);
        }
        Command::Delete(key) => {
            root_map.remove(&key);
        }
    }

    // to_stringしてから返すと非決定的になる
    parsed
}

#[test]
fn test_edit() {
    assert_eq!(
        edit(
            r#"{
                "BIZUDPMincho" : <Single: {"src": "dist/fonts/BIZUDPMincho-Regular.ttf"}>
            }"#,
            Command::Set("BIZUDPGothic".to_string(),
            "dist/fonts/BIZUDPGothic-Regular.ttf".to_string())
        ),
        yojson_rs::parser::parse(r#"{BIZUDPGothic:<Single:{src:"dist/fonts/BIZUDPGothic-Regular.ttf"}>,BIZUDPMincho:<Single:{src:"dist/fonts/BIZUDPMincho-Regular.ttf"}>}"#).unwrap()
    );

    assert_eq!(
        edit(
            r#"{BIZUDPGothic:<Single:{src:"dist/fonts/BIZUDPGothic-Regular.ttf"}>,BIZUDPMincho:<Single:{src:"dist/fonts/BIZUDPMincho-Regular.ttf"}>}"#,
            Command::Delete("BIZUDPGothic".to_string())
        ),
        yojson_rs::parser::parse(
            r#"{BIZUDPMincho:<Single:{src:"dist/fonts/BIZUDPMincho-Regular.ttf"}>}"#
        )
        .unwrap()
    )
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let mut input = String::new();

    std::io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read stdin");

    let command = Command::new(args);

    println!("{}", yojson_rs::to_string(edit(&input, command)))
}
