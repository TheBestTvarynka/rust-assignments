use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

//
// =======================
// Part 1 (Given in task)
// =======================
//

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

//
// =======================
// Part 1: UserRepository
// =======================
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepoError {
    NotFound,
}

// ---- Static dispatch (generics) ----
pub struct UserRepositoryStatic<S> {
    storage: S,
}

impl<S> UserRepositoryStatic<S>
where
    S: Storage<u64, User>,
{
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub fn add(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    pub fn get(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    pub fn update(&mut self, user: User) -> Result<(), RepoError> {
        if self.storage.get(&user.id).is_none() {
            return Err(RepoError::NotFound);
        }
        self.storage.set(user.id, user);
        Ok(())
    }

    pub fn remove(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }
}

// ---- Dynamic dispatch (trait objects) ----
pub struct UserRepositoryDynamic<'a> {
    storage: Box<dyn Storage<u64, User> + 'a>,
}

impl<'a> UserRepositoryDynamic<'a> {
    pub fn new(storage: Box<dyn Storage<u64, User> + 'a>) -> Self {
        Self { storage }
    }

    pub fn add(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    pub fn get(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    pub fn update(&mut self, user: User) -> Result<(), RepoError> {
        if self.storage.get(&user.id).is_none() {
            return Err(RepoError::NotFound);
        }
        self.storage.set(user.id, user);
        Ok(())
    }

    pub fn remove(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }
}

// ---- Simple injectable in-memory storage (used in tests) ----
#[derive(Default)]
struct InMemoryStorage<K, V> {
    map: HashMap<K, V>,
}

impl<K, V> InMemoryStorage<K, V> {
    fn new() -> Self {
        Self { map: HashMap::new() }
    }
}

impl<K, V> Storage<K, V> for InMemoryStorage<K, V>
where
    K: std::hash::Hash + Eq,
{
    fn set(&mut self, key: K, val: V) {
        self.map.insert(key, val);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}

//
// =======================
// Part 2: snippets app upgrade
// =======================
//

fn now_unix_seconds() -> i64 {
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System clock is before UNIX_EPOCH");
    dur.as_secs() as i64
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snippet {
    pub id: u64,
    pub title: String,
    pub code: String,
    pub created_at: i64,
}

impl Snippet {
    pub fn new(id: u64, title: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            code: code.into(),
            created_at: now_unix_seconds(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnippetsError {
    Io(String),
    InvalidEnv(String),
    Db(String),
    Serde(String),
}

impl From<io::Error> for SnippetsError {
    fn from(e: io::Error) -> Self {
        SnippetsError::Io(e.to_string())
    }
}

pub trait SnippetStorage {
    fn upsert(&mut self, snippet: Snippet) -> Result<(), SnippetsError>;
    fn get(&self, id: u64) -> Result<Option<Snippet>, SnippetsError>;
    fn remove(&mut self, id: u64) -> Result<Option<Snippet>, SnippetsError>;
    fn list(&self) -> Result<Vec<Snippet>, SnippetsError>;
}

/// Parse "<PROVIDER>:<PATH>" without touching env (safe for tests).
pub fn parse_storage_spec(raw: &str) -> Result<(String, PathBuf), SnippetsError> {
    let (kind, path) = raw
        .split_once(':')
        .ok_or_else(|| SnippetsError::InvalidEnv("Expected <PROVIDER>:<PATH>".into()))?;

    if path.trim().is_empty() {
        return Err(SnippetsError::InvalidEnv("Path is empty".into()));
    }

    Ok((kind.trim().to_uppercase(), PathBuf::from(path)))
}

mod json_storage {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct SnippetDto {
        id: u64,
        title: String,
        code: String,
        created_at: i64,
    }

    impl From<Snippet> for SnippetDto {
        fn from(s: Snippet) -> Self {
            Self {
                id: s.id,
                title: s.title,
                code: s.code,
                created_at: s.created_at,
            }
        }
    }

    impl From<SnippetDto> for Snippet {
        fn from(d: SnippetDto) -> Self {
            Self {
                id: d.id,
                title: d.title,
                code: d.code,
                created_at: d.created_at,
            }
        }
    }

    pub struct JsonFileSnippetStorage {
        path: PathBuf,
    }

    impl JsonFileSnippetStorage {
        pub fn new(path: impl Into<PathBuf>) -> Self {
            Self { path: path.into() }
        }

        fn read_all(&self) -> Result<Vec<Snippet>, SnippetsError> {
            if !self.path.exists() {
                return Ok(Vec::new());
            }
            let raw = fs::read_to_string(&self.path)?;
            if raw.trim().is_empty() {
                return Ok(Vec::new());
            }
            let dto: Vec<SnippetDto> =
                serde_json::from_str(&raw).map_err(|e| SnippetsError::Serde(e.to_string()))?;
            Ok(dto.into_iter().map(Into::into).collect())
        }

        fn write_all(&self, snippets: &[Snippet]) -> Result<(), SnippetsError> {
            if let Some(parent) = self.path.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent)?;
                }
            }
            let dto: Vec<SnippetDto> = snippets.iter().cloned().map(Into::into).collect();
            let raw = serde_json::to_string_pretty(&dto)
                .map_err(|e| SnippetsError::Serde(e.to_string()))?;
            fs::write(&self.path, raw)?;
            Ok(())
        }
    }

    impl SnippetStorage for JsonFileSnippetStorage {
        fn upsert(&mut self, snippet: Snippet) -> Result<(), SnippetsError> {
            let mut all = self.read_all()?;
            if let Some(pos) = all.iter().position(|s| s.id == snippet.id) {
                all[pos] = snippet;
            } else {
                all.push(snippet);
            }
            self.write_all(&all)
        }

        fn get(&self, id: u64) -> Result<Option<Snippet>, SnippetsError> {
            let all = self.read_all()?;
            Ok(all.into_iter().find(|s| s.id == id))
        }

        fn remove(&mut self, id: u64) -> Result<Option<Snippet>, SnippetsError> {
            let mut all = self.read_all()?;
            let idx = all.iter().position(|s| s.id == id);
            let removed = idx.map(|i| all.remove(i));
            self.write_all(&all)?;
            Ok(removed)
        }

        fn list(&self) -> Result<Vec<Snippet>, SnippetsError> {
            self.read_all()
        }
    }
}

mod sqlite_storage {
    use super::*;
    use rusqlite::{params, Connection};

    pub struct SqliteSnippetStorage {
        conn: Connection,
    }

    impl SqliteSnippetStorage {
        pub fn new(path: &Path) -> Result<Self, SnippetsError> {
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent)?;
                }
            }

            let conn = Connection::open(path).map_err(|e| SnippetsError::Db(e.to_string()))?;
            let s = Self { conn };
            s.init()?;
            Ok(s)
        }

        fn init(&self) -> Result<(), SnippetsError> {
            self.conn
                .execute(
                    r#"
                    CREATE TABLE IF NOT EXISTS snippets (
                        id          INTEGER PRIMARY KEY,
                        title       TEXT NOT NULL,
                        code        TEXT NOT NULL,
                        created_at  INTEGER NOT NULL
                    );
                    "#,
                    [],
                )
                .map_err(|e| SnippetsError::Db(e.to_string()))?;
            Ok(())
        }
    }

    impl SnippetStorage for SqliteSnippetStorage {
        fn upsert(&mut self, snippet: Snippet) -> Result<(), SnippetsError> {
            self.conn
                .execute(
                    r#"
                    INSERT INTO snippets (id, title, code, created_at)
                    VALUES (?1, ?2, ?3, ?4)
                    ON CONFLICT(id) DO UPDATE SET
                        title = excluded.title,
                        code = excluded.code,
                        created_at = excluded.created_at;
                    "#,
                    params![snippet.id, snippet.title, snippet.code, snippet.created_at],
                )
                .map_err(|e| SnippetsError::Db(e.to_string()))?;
            Ok(())
        }

        fn get(&self, id: u64) -> Result<Option<Snippet>, SnippetsError> {
            let mut stmt = self
                .conn
                .prepare("SELECT id, title, code, created_at FROM snippets WHERE id = ?1")
                .map_err(|e| SnippetsError::Db(e.to_string()))?;

            let mut rows = stmt
                .query(params![id])
                .map_err(|e| SnippetsError::Db(e.to_string()))?;

            if let Some(row) = rows.next().map_err(|e| SnippetsError::Db(e.to_string()))? {
                Ok(Some(Snippet {
                    id: row.get(0).map_err(|e| SnippetsError::Db(e.to_string()))?,
                    title: row.get(1).map_err(|e| SnippetsError::Db(e.to_string()))?,
                    code: row.get(2).map_err(|e| SnippetsError::Db(e.to_string()))?,
                    created_at: row.get(3).map_err(|e| SnippetsError::Db(e.to_string()))?,
                }))
            } else {
                Ok(None)
            }
        }

        fn remove(&mut self, id: u64) -> Result<Option<Snippet>, SnippetsError> {
            let existing = self.get(id)?;
            if existing.is_none() {
                return Ok(None);
            }
            self.conn
                .execute("DELETE FROM snippets WHERE id = ?1", params![id])
                .map_err(|e| SnippetsError::Db(e.to_string()))?;
            Ok(existing)
        }

        fn list(&self) -> Result<Vec<Snippet>, SnippetsError> {
            let mut stmt = self
                .conn
                .prepare("SELECT id, title, code, created_at FROM snippets ORDER BY id ASC")
                .map_err(|e| SnippetsError::Db(e.to_string()))?;

            let rows = stmt
                .query_map([], |row| {
                    Ok(Snippet {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        code: row.get(2)?,
                        created_at: row.get(3)?,
                    })
                })
                .map_err(|e| SnippetsError::Db(e.to_string()))?;

            let mut out = Vec::new();
            for r in rows {
                out.push(r.map_err(|e| SnippetsError::Db(e.to_string()))?);
            }
            Ok(out)
        }
    }
}

pub fn snippet_storage_from_env() -> Result<Box<dyn SnippetStorage>, SnippetsError> {
    let raw = env::var("SNIPPETS_APP_STORAGE")
        .map_err(|_| SnippetsError::InvalidEnv("SNIPPETS_APP_STORAGE is not set".into()))?;

    let (kind, p) = parse_storage_spec(&raw)?;

    match kind.as_str() {
        "JSON" => Ok(Box::new(json_storage::JsonFileSnippetStorage::new(p))),
        "SQLITE" => {
            let provider = sqlite_storage::SqliteSnippetStorage::new(&p)?;
            Ok(Box::new(provider))
        }
        other => Err(SnippetsError::InvalidEnv(format!(
            "Unknown provider: {other}. Use JSON or SQLITE."
        ))),
    }
}

fn main() {
    match env::var("SNIPPETS_APP_STORAGE") {
        Ok(v) => {
            println!("SNIPPETS_APP_STORAGE={v}");
            match snippet_storage_from_env() {
                Ok(_) => println!("Storage provider initialized successfully."),
                Err(e) => eprintln!("Failed to init provider: {e:?}"),
            }
        }
        Err(_) => {
            println!("SNIPPETS_APP_STORAGE is not set.");
            println!("Example:");
            println!("  JSON:/tmp/snippets.json");
            println!("  SQLITE:/tmp/snippets.sqlite");
        }
    }
}

//
// =======================
// Tests
// =======================
//

#[cfg(test)]
mod tests {
    use super::*;

    fn user1() -> User {
        User {
            id: 1,
            email: Cow::Borrowed("a@b.com"),
            activated: false,
        }
    }

    fn user2() -> User {
        User {
            id: 2,
            email: Cow::Borrowed("x@y.com"),
            activated: true,
        }
    }

    #[test]
    fn static_repo_add_get_update_remove() {
        let storage = InMemoryStorage::<u64, User>::new();
        let mut repo = UserRepositoryStatic::new(storage);

        assert_eq!(repo.get(1), None);

        repo.add(user1());
        assert_eq!(repo.get(1), Some(&user1()));

        let mut u = user1();
        u.activated = true;
        repo.update(u.clone()).unwrap();
        assert_eq!(repo.get(1), Some(&u));

        let removed = repo.remove(1);
        assert_eq!(removed, Some(u));
        assert_eq!(repo.get(1), None);
    }

    #[test]
    fn static_repo_update_nonexistent_is_error() {
        let storage = InMemoryStorage::<u64, User>::new();
        let mut repo = UserRepositoryStatic::new(storage);

        let res = repo.update(user2());
        assert_eq!(res, Err(RepoError::NotFound));
    }

    #[test]
    fn dynamic_repo_add_get_update_remove() {
        let storage = InMemoryStorage::<u64, User>::new();
        let mut repo = UserRepositoryDynamic::new(Box::new(storage));

        assert_eq!(repo.get(2), None);

        repo.add(user2());
        assert_eq!(repo.get(2), Some(&user2()));

        let mut u = user2();
        u.email = Cow::Borrowed("new@email.com");
        repo.update(u.clone()).unwrap();
        assert_eq!(repo.get(2), Some(&u));

        let removed = repo.remove(2);
        assert_eq!(removed, Some(u));
        assert_eq!(repo.get(2), None);
    }

    #[test]
    fn dynamic_repo_update_nonexistent_is_error() {
        let storage = InMemoryStorage::<u64, User>::new();
        let mut repo = UserRepositoryDynamic::new(Box::new(storage));

        let res = repo.update(user1());
        assert_eq!(res, Err(RepoError::NotFound));
    }

    #[test]
    fn snippet_new_sets_created_at() {
        let s = Snippet::new(1, "t", "code");
        assert!(s.created_at > 1_000_000_000);
    }

    #[test]
    fn parse_storage_spec_rejects_bad_format() {
        let r = parse_storage_spec("BADFORMAT");
        assert!(r.is_err());
    }
}
