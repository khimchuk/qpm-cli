use std::ops::Div;
use std::env;
use std::path::Path;
use std::fs;
use std::process;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let user_home_dir = match home::home_dir() {
        Some(path) => path,
        None => {
            println!("Opening error: *** Can't open your home dir.");
            process::exit(1);
        }
    };

    let storage_dir_exists = Path::new(&user_home_dir.join(".qpass_storage")).exists();
    
    if !storage_dir_exists {
        if let Err(err) = fs::create_dir_all(user_home_dir.join(".qpass_storage")) {
            println!("Creation error: *** {}.", err);
            process::exit(1);
        }
    }

    let conn = Connection::open(user_home_dir.join(".qpass_storage").join("qpass.db"))?;

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
        println!("Parsing error: *** {}.", error);
        help(None);
        process::exit(1);
    });

    run(config, conn);

    Ok(())
}

fn run(config: Config, conn: Connection) {
    match config.operation.as_str() {
        "--help" | "-h" => help(config.input),
        "--version" | "-v" => {
            version();
            if config.input != None {
                println!();
                help(None);
            }
        },
        "--get" | "-g" => {
            if config.input != None {
                println!("Parsing error: *** Too many arguments.");
                help(std::option::Option::Some("--get".to_string()));
                process::exit(1);
            } else {
                get(conn).unwrap()
            }
        },
        "--set" | "-s" => {
            if config.input == None {
                println!("Parsin error: *** No arguments were passed.");
                help(std::option::Option::Some("--set".to_string()));
                process::exit(1);
            } else {
                set(conn, config.input.unwrap()).unwrap();
            }
        },
        "--delete" | "-d" => {
            if config.input != None {
                println!("Parsing error: *** Too many arguments.");
                help(std::option::Option::Some("--delete".to_string()));
                process::exit(1);
            } else {
                delete(conn).unwrap()
            }
        }
        "--list" | "-l" => {
            if config.input != None {
                println!("Parsing error: *** Too many arguments.");
                help(std::option::Option::Some("--list".to_string()));
                process::exit(1);
            } else {
                list(&conn).unwrap();
            }
        },
        unknow => {
            println!("Parsing error: *** Unknown option '{}'.", unknow);
            help(None);
            process::exit(1);
        }
    }

    process::exit(0);
}

fn help(argument: Option<String>) {
    if argument == None {
            println!(
"Usage: qpass [OPTION]
       qpass [OPTION] [ARGUMENT]

Options:
    -h, --help              help message
    -v, --version           qpass version

    -s, --set [NAME]        set password 
    -g, --get               get password
    -d, --delete            remove password
    -l, --list              get all apps names

Type for more information:
    qpass --help [OPTION]
    qpass -h [OPTION]

Report bugs to <khimchuk.io@gmail.com>"
);
            process::exit(0);
    }

    match argument.unwrap().as_str() {
        "--get" | "-g" => {
            println!(
"Usage: qpass --get
       qpass -g"
);
        },
        "--set" | "-s" => {
            println!(
"Usage: qpass --set [NAME]
       qpass -s [NAME]"
);
        },
        "--delete" | "-d" => {
            println!(
"Usage: qpass --delete  
       qpass -d"
);
        },
        "--list" | "-l" => {
            println!(
"Usage: qpass --list 
       qpass -l"
);
        },
        _ => {
            println!("Input error: *** Unknow argument.");
            help(None);
        }
    }
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
        match args.len() {
            2 => {
                let operation = args[1].clone();
                Ok(Config { operation, input: None })
            },
            3 => {
                let operation = args[1].clone();
                let input = Some(args[2].clone());
                
                Ok(Config { operation, input })
            },
            4.. => {
                Err("Too many arguments")
            },
            _ => {
                Err("No options were passed")
            }
        }
    }
}

fn list(conn: &Connection) -> Result<bool, rusqlite::Error>{
    let mut cursor = conn.prepare("SELECT id, name FROM passwords")?;
    let rows = cursor.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?))
    })?;

    println!("+-----+-------------------------+");
    println!("| id: | name:                   |");
    println!("+-----+-------------------------+");
    let mut empty = true;
    for row in rows {
        let (id, name): (i32, String) = row?;
        if name.chars().count() > 23 {
            println!("| {:<3} | {:<23} |", id, &name[0..23]);
            let mut left = name.chars().count() - 23;
            let mut pos = 23;
            for _ in 0..(name.chars().count().div(23)) {
                if left > 23 {
                    println!("|     | {:<23} |", &name[pos..(pos + 23)]);
                    left -= 23;
                } else {
                    println!("|     | {:<23} |", &name[pos..]);
                }
                pos += 23;
            }
            empty = false;
        } else {
            println!("| {:<3} | {:<23} |", id, name);
            empty = false
        }
    }

    if empty {
        println!("|  E M P T Y    F O R    N O W  |");
    }

    println!("+-----+-------------------------+");

    Ok(empty)
}

fn get(conn: Connection) -> Result<(), rusqlite::Error> {
    if let Ok(true) = list(&conn) {
        process::exit(0);
    };

    let id = rpassword::prompt_password("Choose ID: ").unwrap_or_else(|err| {
        println!();
        println!("Input error: *** {}.", err);
        process::exit(1);
    });

    let mut cursor = conn.prepare("SELECT password FROM passwords WHERE id=?")?;
    let rows = cursor.query_map([id], |row| {
        Ok(row.get(0)?)
    })?;
    
    let mut password = String::new();

    for row in rows {
        (password) = row?;
    }

    if password.len() == 0 {
        println!("Input error: *** A password with this ID does not exist.");
        process::exit(1);
    }

    let secret = match rpassword::prompt_password("Secret: ") {
       Ok(line) => {
           if line.chars().count() != 0 {
               line
           } else {
               println!("*** Canceled. ***");
               process::exit(1);
           }
       }
       Err(_) => {
           println!();
           println!("*** Canceled. ***");
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
                println!("*** Canceled. ***");
                process::exit(1);
            }
        }
        Err(_) => {
            println!();
            println!("*** Canceled. ***");
            process::exit(1);
        }
    };

    let mut password = match rpassword::prompt_password("Password: ") {
        Ok(line) => {
            if line.chars().count() != 0 {
                line
            } else {
                println!("*** Canceled. ***");
                process::exit(1);
            }
        }
        Err(_) => {
            println!();
            println!("*** Canceled. ***");
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
    
    println!("*** Success. ***");
    Ok(())
}

fn delete(conn: Connection) -> Result<(), rusqlite::Error> {
    if let Ok(true) = list(&conn) {
        process::exit(0);
    };

    let id = rpassword::prompt_password("Choose id: ").unwrap_or_else(|err| {
        println!();
        println!("Input error: *** {}.", err);
        process::exit(1);
    });

    if let Ok(0) = conn.execute("DELETE FROM passwords WHERE id=?",
        [&id],
    ) {
        println!("Input error: *** A password with this ID does not exist.");
        process::exit(1);
    };

    conn.execute("UPDATE passwords SET id = id - 1 WHERE id > ?",
        [&id],
    )?;
    
    println!("*** Success. ***");
    Ok(())
}

fn version() {
    println!("Quick Password Manager {}", env!("CARGO_PKG_VERSION"));
}
