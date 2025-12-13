mod part_1;
mod part_2;

use std::collections::HashMap;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use serde::{Deserialize, Serialize};

const FILE_PATH: &str = "snippets.json";

#[derive(Serialize, Deserialize, Debug)]
struct Snippets {
    data: HashMap<String, String>,
}

impl Snippets {
    fn load() -> Self {
        if let Ok(contents) = fs::read_to_string(FILE_PATH) {
            serde_json::from_str(&contents).unwrap_or_else(|_| Snippets {
                data: HashMap::new(),
            })
        } else {
            Snippets {
                data: HashMap::new(),
            }
        }
    }

    fn save(&self) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(FILE_PATH)
            .unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    fn add(&mut self, name: &str, code: String) {
        self.data.insert(name.to_string(), code);
        self.save();
    }

    fn read(&self, name: &str) {
        if let Some(code) = self.data.get(name) {
            println!("{}", code);
        } else {
            eprintln!("Snippet '{}' not found", name);
        }
    }

    fn delete(&mut self, name: &str) {
        if self.data.remove(name).is_some() {
            println!("Deleted '{}'", name);
            self.save();
        } else {
            eprintln!("Snippet '{}' not found", name);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut snippets = Snippets::load();

    if let Some(pos) = args.iter().position(|a| a == "--name") {
        if let Some(name) = args.get(pos + 1) {
            let mut code = String::new();
            io::stdin().read_to_string(&mut code).unwrap();
            snippets.add(name, code);
            println!("Snippet '{}' saved.", name);
            return;
        }
    }

    if let Some(pos) = args.iter().position(|a| a == "--read") {
        if let Some(name) = args.get(pos + 1) {
            snippets.read(name);
            return;
        }
    }

    if let Some(pos) = args.iter().position(|a| a == "--delete") {
        if let Some(name) = args.get(pos + 1) {
            snippets.delete(name);
            return;
        }
    }

    eprintln!("Usage:
  echo \"code\" | snippets-app --name \"Snippet Name\"
  snippets-app --read \"Snippet Name\"
  snippets-app --delete \"Snippet Name\"");
}
