
# Assignment 3: Polymorphism (static and dynamic dispatch).

## Part 1

Given the following `Storage` abstraction and `User` entity:

```rust
trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}
```

Implement `UserRepository` type with injectable `Storage` implementation, which can get, add, update and remove `User` in the injected `Storage`.
Make two different implementations: one should use _dynamic dispatch_ (trait objects) for `Storage` injecting, and the other one should use _static dispatch_ (generics).
Prove your implementation correctness with tests.

_Note 1: You **are not allowed** to change the `Storage` traits or `User` struct definition._

_Note 2: Injectable. What does it mean?_



## Part 2

Remember the snippets-app from the previous assignment? Good :smiling_imp:. In this part, you will improve the previous snippets app implementation. The list of new requirements:

1. I often want to know when I created the snippet. So, here is my request: record the creation time when creating the snippet.
2. Add a new storage option: [SQLite](https://sqlite.org/index.html) database.
   Your app should read the `SNIPPETS_APP_STORAGE` environment variable and use the storage provider depending on this environment variable content.
   The `SNIPPETS_APP_STORAGE` value should have the following pattern: `<storage provider name>:<path to file>`. Your app must support two storage providers: `JSON` and `SQLITE`.
   Here are a few examples:
   | `SNIPPETS_APP_STORAGE` value example | meaning |
   |-|-|
   | `JSON:/home/pavlo/snippets.json` | The app should use the `/home/pavlo/snippets.json` file to store all code snippets. |
   | `SQLITE:/home/pavlo/snippets.sqlite` | The app should use the `/home/pavlo/snippets.sqlite` as SQLite database file. |

