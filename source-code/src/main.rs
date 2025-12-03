use std::env;
use std::fs::File;
use std::io::{self, Read};
use serde_json::{self, Value};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage:");
        println!("  json_handler pretty <file or ->");
        println!("  json_handler get <path> <file or ->");
        println!("Path example: .key.subkey[0]");
        return Ok(());
    }
    let command = &args[1];
    let input = &args[args.len() - 1];
    let mut data = String::new();
    if input == "-" {
        io::stdin().read_to_string(&mut data)?;
    } else {
        let mut file = File::open(input)?;
        file.read_to_string(&mut data)?;
    }
    let value: Value = serde_json::from_str(&data).unwrap_or(Value::Null);
    match command.as_str() {
        "pretty" => {
            let pretty = serde_json::to_string_pretty(&value).unwrap_or_default();
            println!("{}", pretty);
        }
        "get" => {
            if args.len() < 4 {
                println!("Usage: json_handler get <path> <file or ->");
                return Ok(());
            }
            let path = &args[2];
            let mut current = &value;
            for part in path.split('.') {
                if part.is_empty() {
                    continue;
                }
                if let Some(index_str) = part.strip_suffix(']') {
                    if let Some(index_str) = index_str.strip_prefix('[') {
                        if let Ok(index) = index_str.parse::<usize>() {
                            if let Value::Array(arr) = current {
                                current = arr.get(index).unwrap_or(&Value::Null);
                            } else {
                                current = &Value::Null;
                            }
                        } else {
                            current = &Value::Null;
                        }
                    } else {
                        if let Value::Object(map) = current {
                            current = map.get(part).unwrap_or(&Value::Null);
                        } else {
                            current = &Value::Null;
                        }
                    }
                } else {
                    if let Value::Object(map) = current {
                        current = map.get(part).unwrap_or(&Value::Null);
                    } else {
                        current = &Value::Null;
                    }
                }
            }
            println!("{}", serde_json::to_string(current).unwrap_or_default());
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
    Ok(())
}
