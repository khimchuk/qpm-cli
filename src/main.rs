use std::env;
use std::path::Path;
use std::process;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let user_home_dir = match home::home_dir() {
        Some(path) => path,
        None => {
            println!("Can't open your home dir!");
            process::exit(1);
        }
    };

    let path_exists = Path::new(&user_home_dir.join(".scrappy_storage")).exists();
    
    if path_exists == false {
        println!(
"Oops! Looks like Scrappy can't find the storage directory. Try creating it using the command:
    --> mkdir ~/.scrappy_storage 
Or reinstall Scrappy by running the install.sh script");
        process::exit(1);
    }

    let conn = Connection::open(user_home_dir.join(".scrappy_storage").join("scrappy.db"))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS passwords (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
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

fn run(config: Config, conn: Connection) {
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

fn encryption(password: String) -> String {
    let mut result: String = String::new();

    for byte in password.chars() {
        let binary_repr = format!("{:08b}", byte as u8);
        result.push_str(&binary_repr);
    }

    return result;
}

fn decryption(to_encrypt: String) -> String {
    let mut result: String = String::new();

    for chunk in to_encrypt.chars().collect::<Vec<_>>().chunks(8) {
        let byte: String = chunk.iter().collect();
        let decrypted_byte: u8 = u8::from_str_radix(&byte, 2).unwrap();
        result.push(decrypted_byte as char);
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

fn get_all(conn: Connection) -> Result<(), rusqlite::Error>{
    let mut cursor = conn.prepare("SELECT id, name FROM passwords")?;
    let rows = cursor.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?))
    })?;

    println!("+------+------------------------+");
    println!("|  id  |          name          |");
    println!("+------+------------------------+");
    for row in rows {
        let (id, name): (i32, String) = row?;
        println!("|  {:<2}  | {:<22} |", id, name);
    }
    println!("+------+------------------------+");

    Ok(())
}

fn get(conn: Connection, id: &str) -> Result<(), rusqlite::Error> {
    let mut cursor = conn.prepare("SELECT password FROM passwords WHERE id=?")?;
    let rows = cursor.query_map([id.to_string()], |row| {
        Ok(row.get(0)?)
    })?;
    
    let mut password = String::new();

    for row in rows {
        (password) = row?;
    }

    let decrypted_password = decryption(password);

    println!("{}", decrypted_password);

    Ok(())
}
