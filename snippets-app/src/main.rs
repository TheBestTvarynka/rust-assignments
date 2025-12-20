#![deny(
    missing_docs,
    broken_intra_doc_links,
    missing_crate_level_docs,
    unreachable_pub,
    clippy::missing_panics_doc,
    clippy::clone_on_ref_ptr,
    clippy::similar_names
)]

use std::env;
use std::process;

fn main() {
if let Err(err) = run() {
        eprintln!("{err}");
    process::exit(1);
    }
  }

fn run() -> Result<(), AppError> {
         let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
          return Err(AppError::InvalidArguments);
}

    let storage = Storage::from_env()?;
    let command = &args[1];
    let name = &args[2];

    match command.as_str() {
        "--name" => storage.create(name),
        "--read" => storage.read(name),
        "--delete" => storage.delete(name),
        _ => Err(AppError::UnknownCommand),
}
}

#[derive(Debug)]
pub enum AppError {
InvalidArguments,
UnknownCommand,
StorageError(String),
}

pub struct Storage {
backend: Box<dyn StorageBackend>,
}

impl Storage {
    pub fn from_env() -> Result<Self, AppError> {
        let value =
            env::var("SNIPPETS_APP_STORAGE").map_err(|_| AppError::StorageError("env".into()))?;

        let (kind, path) = value
            .split_once(':')
            .ok_or_else(|| AppError::StorageError("format".into()))?;

        let backend: Box<dyn StorageBackend> = match kind {
            "JSON" => Box::new(JsonStorage::new(path)),
            "SQLITE" => Box::new(SqliteStorage::new(path)),
            _ => return Err(AppError::StorageError("type".into())),
        };

        Ok(Self { backend })
    }

    pub fn create(&self, name: &str) -> Result<(), AppError> {
        self.backend.create(name)
    }

    pub fn read(&self, name: &str) -> Result<(), AppError> {
        self.backend.read(name)
    }

    pub fn delete(&self, name: &str) -> Result<(), AppError> {
        self.backend.delete(name)
    }
}

pub trait StorageBackend {
    fn create(&self, name: &str) -> Result<(), AppError>;
    fn read(&self, name: &str) -> Result<(), AppError>;
    fn delete(&self, name: &str) -> Result<(), AppError>;
}

pub struct JsonStorage;

impl JsonStorage {
    pub fn new(_path: &str) -> Self {
        Self
    }
}

impl StorageBackend for JsonStorage {
    fn create(&self, _name: &str) -> Result<(), AppError> {
        Ok(())
    }
    fn read(&self, _name: &str) -> Result<(), AppError> {
        Ok(())
    }
    fn delete(&self, _name: &str) -> Result<(), AppError> {
        Ok(())
    }
}

pub struct SqliteStorage;

impl SqliteStorage {
    pub fn new(_path: &str) -> Self {
        Self
    }
}

impl StorageBackend for SqliteStorage {
    fn create(&self, _name: &str) -> Result<(), AppError> {
        Ok(())
    }
    fn read(&self, _name: &str) -> Result<(), AppError> {
        Ok(())
    }
    fn delete(&self, _name: &str) -> Result<(), AppError> {
        Ok(())
    }
}
