use std::ops::Div;
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
        "--get" | "-g" => get(conn).unwrap(),
        "--set" | "-s" => set(conn, config.input.unwrap()).unwrap(),
        "--get-all" | "-ga" if config.input == None => get_all(&conn).unwrap(),
        _ => println!("Problem parsing arguments: Unknow arguments")
    }

    process::exit(0);
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

fn get_all(conn: &Connection) -> Result<(), rusqlite::Error>{
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

fn get(conn: Connection) -> Result<(), rusqlite::Error> {
    let _ = get_all(&conn);

    let id = rpassword::prompt_password("Choose id: ").unwrap();

    let mut cursor = conn.prepare("SELECT password FROM passwords WHERE id=?")?;
        let rows = cursor.query_map([id], |row| {
            Ok(row.get(0)?)
        })?;
    
    let mut password = String::new();

    for row in rows {
        (password) = row?;
    }

    if password.len() == 0 {
        println!("A password with this ID does not exist!");
        process::exit(1);
    }

    let secret = match rpassword::prompt_password("Secret: ") {
       Ok(line) => {
           if line.chars().count() != 0 {
               line
           } else {
               println!("Canceled");
               process::exit(1);
           }
       }
       Err(_) => {
           println!();
           println!("Canceled");
           process::exit(1);
       }
    };

    let mut new_secret = String::new();
 
    for _ in 0..(password.chars().count() / 8).div(secret.chars().count()) {
        new_secret.push_str(secret.as_str());
    }

    new_secret.push_str(&secret[..(password.chars().count() / 8) % secret.chars().count()]);
    
    let mut new_password = String::new();

    new_secret = encryption(new_secret);

    for index in 0..password.chars().count() {
        if new_secret.chars().nth(index).unwrap() != password.chars().nth(index).unwrap() {
            new_password.push('1');
        } else {
            new_password.push('0');
        } 
    }

    new_password = decryption(new_password);

    println!("{}", new_password);

    Ok(())
}

fn set(conn: Connection, name: String) -> Result<(), rusqlite::Error> {
    let secret = match rpassword::prompt_password("Secret: ") {
        Ok(line) => {
            if line.chars().count() != 0 {
                line
            } else {
                println!("Canceled");
                process::exit(1);
            }
        }
        Err(_) => {
            println!();
            println!("Canceled");
            process::exit(1);
        }
    };

    let mut password = match rpassword::prompt_password("Password: ") {
        Ok(line) => {
            if line.chars().count() != 0 {
                line
            } else {
                println!("Canceled");
                process::exit(1);
            }
        }
        Err(_) => {
            println!();
            println!("Canceled");
            process::exit(1);
        }
    };

    let mut new_secret = String::new();

    for _ in 0..password.chars().count().div(secret.chars().count()) {
        new_secret.push_str(secret.as_str());
    }

    new_secret.push_str(&secret[..password.chars().count() % secret.chars().count()]);

    new_secret = encryption(new_secret);
    password = encryption(password);
    
    let mut new_password = String::new();
    
    for index in 0..password.chars().count() {
        if new_secret.chars().nth(index).unwrap() != password.chars().nth(index).unwrap() {
            new_password.push('1');
        } else {
            new_password.push('0');
        } 
    }

    conn.execute("INSERT INTO passwords(name, password) VALUES(?1, ?2)",
        (name, new_password),
    )?;

    Ok(())
}
