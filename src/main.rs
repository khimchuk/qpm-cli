use std::env;
use std::process;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("scrappy.db")?;

    create_table(conn);

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    run(config);

    Ok(())
}

fn run(config: Config) {
    if (config.operation == "--help" || config.operation == "-h") && config.input == None {
        help();
    } else if config.operation == "--encryption" || config.operation == "-e" {
        println!("{}", encryption(config.input.unwrap()));
    } else if config.operation == "--decryption" || config.operation == "-d" {
        println!("{}", decryption(config.input.unwrap()));
    } else {
        println!("Problem parsing arguments: Unknow argument");
        process::exit(1);
    }
}

fn help() {
    println!("Usage:
  scrappy [OPTION] - for help command
  scrappy [OPTION] [ARGUMENT] - for other commands

  -h, --help              help message
  -e, --encryption        encrypt message
  -d, --decryption        decrypt message
");
}

fn encryption(message: String) -> String {
    let mut result: String = String::new();

    for element in message.chars().rev(){
        result.push(element);
    }

    return result;
    
}

fn decryption(message: String) -> String {
    let mut result: String = String::new();

    for element in message.chars().rev() {
        result.push(element);
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

fn create_table(connection: Connection) {
    let _ = connection.execute(
        "CREATE TABLE IF NOT EXISTS passwords (
            name TEXT,
            password TEXT
        )",
        (),
    );
}
