use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, Read, Write},
    path::Path,
};

const STORAGE: &str = "snippets.txt";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage:");
        eprintln!("  --name <NAME>");
        eprintln!("  --read <NAME>");
        eprintln!("  --delete <NAME>");
        return;
    }

    let command = &args[1];
    let name = &args[2];

    match command.as_str() {
        "--name" => create_snippet(name),
        "--read" => read_snippet(name),
        "--delete" => delete_snippet(name),
        _ => eprintln!("Unknown command"),
    }
}

fn create_snippet(name: &str) {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(STORAGE)
        .unwrap();

    writeln!(file, "###{}\n{}", name, input).unwrap();
}

fn read_snippet(name: &str) {
    if !Path::new(STORAGE).exists() {
        return;
    }

    let content = fs::read_to_string(STORAGE).unwrap();

    for block in content.split("###").skip(1) {
        if block.starts_with(name) {
            let code = block[name.len()..].trim();
            println!("{}", code);
            return;
        }
    }

    eprintln!("Snippet not found");
}

fn delete_snippet(name: &str) {
    if !Path::new(STORAGE).exists() {
        return;
    }

    let content = fs::read_to_string(STORAGE).unwrap();
    let new_content: String = content
        .split("###")
        .filter(|b| !b.starts_with(name))
        .map(|b| format!("###{}", b))
       .collect();

    fs::write(STORAGE, new_content).unwrap();
}
