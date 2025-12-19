use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

// PART 1
trait Storage<K, V> {
fn set(&mut self, key: K, val: V);
fn get(&self, key: &K) -> Option<&V>;
 fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Clone, Debug, PartialEq)]
struct User {
id: u64,
    email: Cow<'static, str>,
activated: bool,
}
struct InMemoryStorage<K, V> {
    data: HashMap<K, V>,
}

impl<K: Eq + std::hash::Hash, V> InMemoryStorage<K, V> {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}
impl<K: Eq + std::hash::Hash, V> Storage<K, V> for InMemoryStorage<K, V> {
    fn set(&mut self, key: K, val: V) {
        self.data.insert(key, val);
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }
}
struct UserRepositoryStatic<S: Storage<u64, User>> {
    storage: S,
}

impl<S: Storage<u64, User>> UserRepositoryStatic<S> {
    fn new(storage: S) -> Self {
        Self { storage }
    }

    fn add(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn get(&self, id: &u64) -> Option<&User> {
        self.storage.get(id)
    }

    fn update(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn remove(&mut self, id: &u64) -> Option<User> {
        self.storage.remove(id)
    }
}

struct UserRepositoryDynamic {
    storage: Box<dyn Storage<u64, User>>,
}
impl UserRepositoryDynamic {
    fn new(storage: Box<dyn Storage<u64, User>>) -> Self {
        Self { storage }
    }

    fn add(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn get(&self, id: &u64) -> Option<&User> {
        self.storage.get(id)
    }

    fn update(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn remove(&mut self, id: &u64) -> Option<User> {
        self.storage.remove(id)
    }
}

// PART 2
#[derive(Clone, Debug)]
struct Snippet {
    id: u64,
    content: String,
    created_at: u64,
}

trait SnippetStorage {
    fn add(&mut self, snippet: Snippet);
    fn list(&self) -> Vec<Snippet>;
}

struct JsonSnippetStorage {
    path: String,
    snippets: Vec<Snippet>,
}

impl JsonSnippetStorage {
    fn new(path: String) -> Self {
        let snippets = if Path::new(&path).exists() {
            let data = fs::read_to_string(&path).unwrap();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Vec::new()
        };

        Self { path, snippets }
    }

    fn persist(&self) {
        let data = serde_json::to_string_pretty(&self.snippets).unwrap();
        fs::write(&self.path, data).unwrap();
    }
}

impl SnippetStorage for JsonSnippetStorage {
    fn add(&mut self, snippet: Snippet) {
        self.snippets.push(snippet);
        self.persist();
    }

    fn list(&self) -> Vec<Snippet> {
        self.snippets.clone()
    }
}

struct SqliteSnippetStorage {
    conn: rusqlite::Connection,
}

impl SqliteSnippetStorage {
    fn new(path: String) -> Self {
        let conn = rusqlite::Connection::open(path).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS snippets (
                id INTEGER PRIMARY KEY,
                content TEXT NOT NULL,
                created_at INTEGER NOT NULL
            )",
            [],
        )
        .unwrap();
        Self { conn }
    }
}

impl SnippetStorage for SqliteSnippetStorage {
    fn add(&mut self, snippet: Snippet) {
        self.conn
            .execute(
                "INSERT INTO snippets (id, content, created_at) VALUES (?1, ?2, ?3)",
                (&snippet.id, &snippet.content, &snippet.created_at),
            )
            .unwrap();
    }

    fn list(&self) -> Vec<Snippet> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, content, created_at FROM snippets")
            .unwrap();

        let rows = stmt
            .query_map([], |row| {
                Ok(Snippet {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })
            .unwrap();

        rows.map(|r| r.unwrap()).collect()
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
fn snippets_storage_from_env() -> Box<dyn SnippetStorage> {
    let value = env::var("SNIPPETS_APP_STORAGE").expect("SNIPPETS_APP_STORAGE not set");
    let (kind, path) = value.split_once(':').expect("Invalid format");

    match kind {
        "JSON" => Box::new(JsonSnippetStorage::new(path.to_string())),
        "SQLITE" => Box::new(SqliteSnippetStorage::new(path.to_string())),
        _ => panic!("Unsupported storage"),
    }
}
fn create_snippet(storage: &mut dyn SnippetStorage, content: String) {
    let ts = current_timestamp();
    storage.add(Snippet {
        id: ts,
        content,
        created_at: ts,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_repo_works() {
        let storage = InMemoryStorage::new();
        let mut repo = UserRepositoryStatic::new(storage);

        let user = User {
            id: 1,
            email: Cow::Borrowed("a@test.com"),
            activated: true,
        };

        repo.add(user.clone());
        assert_eq!(repo.get(&1), Some(&user));
        repo.remove(&1);
        assert!(repo.get(&1).is_none());
    }
    #[test]
    fn dynamic_repo_works() {
        let storage = Box::new(InMemoryStorage::new());
        let mut repo = UserRepositoryDynamic::new(storage);

        let user = User {
            id: 2,
            email: Cow::Borrowed("b@test.com"),
            activated: false,
        };

        repo.add(user.clone());
        assert_eq!(repo.get(&2), Some(&user));
    }
}

fn main() {
    let mut storage = snippets_storage_from_env();
    create_snippet(storage.as_mut(), "example snippet".to_string());
    let snippets = storage.list();
    println!("{:?}", snippets);
}
