use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};

const BIN_PATH: &str = "/home/pat/vs/AC/bin_unix/native_client";
const ENV_PATH: &str = ".env";

/// Run nm on the binary to find the offsets of the required symbols and write the to a .env file
fn main() {
    let required_symbols = ["player1", "players"];

    if !Path::new(ENV_PATH).exists() {
        let mut file = fs::File::create_new(ENV_PATH).expect("Failed to create .env file");

        let bin_path = BIN_PATH;
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
