use anyhow::{Context, Result};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Snippet {
    id: u64,
    content: String,
    created_at: u64,
}

trait SnippetStorage {
    fn add(&mut self, snippet: Snippet) -> Result<()>;
    fn list(&self) -> Result<Vec<Snippet>>;
}

struct JsonStorage {
    path: String,
}

impl JsonStorage {
    fn new(path: String) -> Self {
        Self { path }
    }

    fn read_all(&self) -> Result<Vec<Snippet>> {
        if !Path::new(&self.path).exists() {
            return Ok(Vec::new());
        }

        let data = fs::read_to_string(&self.path)
            .with_context(|| format!("Failed to read file {}", self.path))?;

        let snippets = serde_json::from_str(&data)
            .with_context(|| "Failed to parse JSON snippets")?;

        Ok(snippets)
    }

    fn write_all(&self, snippets: &[Snippet]) -> Result<()> {
        let data = serde_json::to_string_pretty(snippets)
            .with_context(|| "Failed to serialize snippets to JSON")?;

        fs::write(&self.path, data)
            .with_context(|| format!("Failed to write file {}", self.path))?;

        Ok(())
    }
}

impl SnippetStorage for JsonStorage {
    fn add(&mut self, snippet: Snippet) -> Result<()> {
        let mut snippets = self.read_all()?;
        snippets.push(snippet);
        self.write_all(&snippets)
    }

    fn list(&self) -> Result<Vec<Snippet>> {
        self.read_all()
    }
}

struct SqliteStorage {
    conn: Connection,
}

impl SqliteStorage {
    fn new(path: String) -> Result<Self> {
        let conn = Connection::open(&path)
            .with_context(|| format!("Failed to open SQLite database {}", path))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS snippets (
                id INTEGER PRIMARY KEY,
                content TEXT NOT NULL,
                created_at INTEGER NOT NULL
            )",
            [],
        )
        .with_context(|| "Failed to create snippets table")?;

        Ok(Self { conn })
    }
}

impl SnippetStorage for SqliteStorage {
    fn add(&mut self, snippet: Snippet) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO snippets (id, content, created_at) VALUES (?1, ?2, ?3)",
                (&snippet.id, &snippet.content, &snippet.created_at),
            )
            .with_context(|| "Failed to insert snippet into SQLite")?;
        Ok(())
    }

    fn list(&self) -> Result<Vec<Snippet>> {
        let mut stmt = self.conn
            .prepare("SELECT id, content, created_at FROM snippets")
            .with_context(|| "Failed to prepare select query")?;

        let rows = stmt
            .query_map([], |row| {
                Ok(Snippet {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })
            .with_context(|| "Failed to read rows from SQLite")?;

        let mut snippets = Vec::new();
        for row in rows {
            snippets.push(row?);
        }

        Ok(snippets)
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn storage_from_env() -> Result<Box<dyn SnippetStorage>> {
    let value = env::var("SNIPPETS_APP_STORAGE")
        .with_context(|| "SNIPPETS_APP_STORAGE environment variable is not set")?;

    let (kind, path) = value
        .split_once(':')
        .with_context(|| "Invalid SNIPPETS_APP_STORAGE format")?;

    match kind {
        "JSON" => Ok(Box::new(JsonStorage::new(path.to_string()))),
        "SQLITE" => Ok(Box::new(SqliteStorage::new(path.to_string())?)),
        _ => Err(anyhow::anyhow!("Unsupported storage type {}", kind)),
    }
}

fn run() -> Result<()> {
    let mut storage = storage_from_env()?;

    let snippet = Snippet {
        id: now(),
        content: "example snippet".to_string(),
        created_at: now(),
    };

    storage.add(snippet)?;
    let _ = storage.list()?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:#}", err);
    }
}
