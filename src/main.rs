use std::env;
use std::process;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("scrappy.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS passwords (
            name TEXT,
            password TEXT
        )",
        (),
    )?;

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    run(config, conn);

    Ok(())
}

fn run(config: Config, _conn: Connection) {
    match config.operation.as_str() {
        "--help" | "-h" if config.input == None => help(),
        _ => println!("Problem parsing arguments: Unknow arguments")
    }

    process::exit(1);
}

fn help() {
    println!("Usage:
  scrappy [OPTION] - for help command
  scrappy [OPTION] [ARGUMENTS] - for other commands

  -h, --help              help message
  -e, --encryption        encrypt message
  -d, --decryption        decrypt message
");
}

fn encryption(password: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for byte in password.as_bytes() {
        let to_add = format!("{:b}", byte);
        result.push(to_add);
    }

    return result;
}

fn decryption(to_encrypt: Vec<String>) -> String {
    let mut result: String = String::new();

    for byte in to_encrypt {
        let to_add = u8::from_str_radix(byte.as_str(), 2).unwrap();
        result.push(to_add as char);
    }

    return result;
}

struct Config {
    operation: String,
    input: Option<String>
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() == 2 {
            let operation = args[1].clone();

            Ok(Config { operation, input: None })
        } else if args.len() == 3 {
            let operation = args[1].clone();
            let input = Some(args[2].clone());

            Ok(Config { operation, input })
        } else if args.len() > 3 {
            return Err("Too many arguments");
        } else {
            return Err("No arguments found");
        }
    }
}
