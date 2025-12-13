use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

//
// =======================
// Snippet model
// =======================
//

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Snippet {
    id: u64,
    title: String,
    code: String,
    created_at: i64,
}

impl Snippet {
    fn new(id: u64, title: String, code: String) -> Result<Self> {
        let created_at = current_unix_time().context("Failed to get snippet creation time")?;
        Ok(Self {
            id,
            title,
            code,
            created_at,
        })
    }
}

fn current_unix_time() -> Result<i64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("System time is before UNIX_EPOCH") // редкий случай, но теперь это не “молчаливый” fallback
        .map(|d| d.as_secs() as i64)
}

//
// =======================
// Storage trait
// =======================
//

trait SnippetStorage {
    fn save(&mut self, snippet: Snippet) -> Result<()>;
    fn list(&self) -> Result<Vec<Snippet>>;
}

//
// =======================
// JSON storage
// =======================
//

struct JsonStorage {
    path: PathBuf,
}

impl JsonStorage {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn read_all(&self) -> Result<Vec<Snippet>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.path)
            .with_context(|| format!("Failed to read JSON file: {:?}", self.path))?;

        if content.trim().is_empty() {
            return Ok(Vec::new());
        }

        let snippets: Vec<Snippet> =
            serde_json::from_str(&content).context("Failed to parse JSON file")?;

        Ok(snippets)
    }

    fn write_all(&self, snippets: &[Snippet]) -> Result<()> {
        // Создаём папку, если её нет (например D:\temp\...)
        if let Some(parent) = self.path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory {:?}", parent))?;
            }
        }

        let json =
            serde_json::to_string_pretty(snippets).context("Failed to serialize snippets to JSON")?;

        fs::write(&self.path, json)
            .with_context(|| format!("Failed to write JSON file: {:?}", self.path))?;

        Ok(())
    }
}

impl SnippetStorage for JsonStorage {
    fn save(&mut self, snippet: Snippet) -> Result<()> {
        let mut snippets = self.read_all().context("Failed to read existing snippets from JSON")?;
        snippets.push(snippet);
        self.write_all(&snippets)
            .context("Failed to write updated snippets to JSON")?;
        Ok(())
    }

    fn list(&self) -> Result<Vec<Snippet>> {
        self.read_all().context("Failed to list snippets from JSON")
    }
}

//
// =======================
// SQLite storage
// =======================
//

struct SqliteStorage {
    conn: Connection,
}

impl SqliteStorage {
    fn new(path: &Path) -> Result<Self> {
        // создаём директорию под файл БД, если нужно
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory {:?}", parent))?;
            }
        }

        let conn = Connection::open(path)
            .with_context(|| format!("Failed to open SQLite DB at {:?}", path))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS snippets (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                code TEXT NOT NULL,
                created_at INTEGER NOT NULL
            )",
            [],
        )
        .context("Failed to create SQLite table")?;

        Ok(Self { conn })
    }
}

impl SnippetStorage for SqliteStorage {
    fn save(&mut self, snippet: Snippet) -> Result<()> {
        // UPSERT: повторный запуск не ломается
        self.conn
            .execute(
                "INSERT INTO snippets (id, title, code, created_at)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(id) DO UPDATE SET
                    title = excluded.title,
                    code = excluded.code,
                    created_at = excluded.created_at;",
                params![snippet.id, snippet.title, snippet.code, snippet.created_at],
            )
            .context("Failed to insert/update snippet in SQLite DB")?;

        Ok(())
    }

    fn list(&self) -> Result<Vec<Snippet>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, code, created_at FROM snippets ORDER BY id ASC")
            .context("Failed to prepare SELECT statement")?;

        let rows = stmt
            .query_map([], |row| {
                Ok(Snippet {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    code: row.get(2)?,
                    created_at: row.get(3)?,
                })
            })
            .context("Failed to query snippets from SQLite DB")?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.context("Failed to read SQLite row")?);
        }

        Ok(result)
    }
}

//
// =======================
// Storage selection
// =======================
//

fn storage_from_env() -> Result<Box<dyn SnippetStorage>> {
    let raw = env::var("SNIPPETS_APP_STORAGE")
        .context("SNIPPETS_APP_STORAGE environment variable is not set")?;

    let (kind, path) = raw
        .split_once(':')
        .context("SNIPPETS_APP_STORAGE must be in format PROVIDER:PATH")?;

    match kind.trim().to_uppercase().as_str() {
        "JSON" => Ok(Box::new(JsonStorage::new(PathBuf::from(path)))),
        "SQLITE" => Ok(Box::new(SqliteStorage::new(Path::new(path))?)),
        other => anyhow::bail!("Unknown storage provider: {other}. Use JSON or SQLITE."),
    }
}

//
// =======================
// Main
// =======================
//

fn main() -> Result<()> {
    let mut storage = storage_from_env().context("Failed to initialize storage backend")?;

    let snippet = Snippet::new(
        1,
        "Example snippet".to_string(),
        "println!(\"Hello from assignment4!\");".to_string(),
    )
    .context("Failed to create snippet")?;

    storage.save(snippet).context("Failed to save snippet")?;

    let snippets = storage.list().context("Failed to list snippets")?;

    println!("Stored snippets:");
    for s in snippets {
        println!("- {} (created at {})", s.title, s.created_at);
    }

    Ok(())
}
