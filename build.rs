use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};

const ENV_FILENAME: &str = ".ac_rs.env";

/// Run nm on the binary to find the offsets of the required symbols and write the to a .env file
fn main() {
    // The symbols to locate and write to the .env file
    let required_symbols = ["player1", "players"];

    // Need a common path to the .env file
    let home = env::var("HOME").expect("HOME environment variable not set");
    let env_path = format!("{}/Documents/{}", home, ENV_FILENAME);

    if !Path::new(&env_path).exists() || fs::metadata(&env_path).is_ok_and(|f| f.len() == 0) {
        let mut file = fs::File::create(&env_path).expect("Failed to create .env file");

        let bin_path = env::var("AC_BIN_PATH").expect("AC_BIN_PATH environment variable not set");
        let output = Command::new("nm")
            .arg(bin_path)
            .output()
            .expect("Failed to execute nm");

        let output = String::from_utf8_lossy(&output.stdout);

        let symbols: Vec<_> = output
            .lines()
            .filter(|line| {
                required_symbols.iter().any(|sym| {
                    let split = line.split_whitespace().collect::<Vec<_>>();
                    split.len() == 3 && split[2] == *sym
                })
            })
            .map(Symbol::new)
            .collect();

        if symbols.len() != required_symbols.len() {
            panic!("Failed to find all required symbols");
        }

        for symbol in symbols {
            symbol.append_to_env(&mut file);
        }
    }
}

struct Symbol {
    name: String,
    offset: u64,
}

impl Symbol {
    fn new(line: &str) -> Self {
        let parts: Vec<_> = line.split_whitespace().collect();
        match parts.as_slice() {
            [offset, _, name] => Self {
                name: name.to_string(),
                offset: u64::from_str_radix(offset, 16).expect("Failed to parse offset"),
            },
            _ => panic!("Invalid symbol line: {}", line),
        }
    }

    fn append_to_env(&self, file: &mut File) {
        let content = format!("{}_OFFSET={:X}\n", self.name.to_uppercase(), self.offset);
        file.write_all(content.as_bytes())
            .expect("Failed to write to .env file");
    }
}
