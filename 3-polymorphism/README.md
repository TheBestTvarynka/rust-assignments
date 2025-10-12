
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

Implement `UserRepository` type with injectable `Storage` implementation, which can get, add, update, and remove `User` in the injected `Storage`.
Make two different implementations: one should use _dynamic dispatch_ (trait objects) for `Storage` injection, and the other one should use _static dispatch_ (generics).
Prove your implementation correctness with tests.

_Note 1: You **are not allowed** to change the `Storage` traits or `User` struct definition._

_Note 2: Injectable: what does it mean?_

_Injectable_ means that you can inject any `Storage` trait implementation inside `UserRepository` and it will work. Your solutions should have two `UserRepository` types (for example, `UserRepositoryStatic` and `UserRepositoryDynamic`).

_Note 3: Place your implementation in the `./src/main.rs` file._

## Part 2

Remember the snippets-app from the previous assignment? Good :smiling_imp:. In this part, you will improve the previous snippets app implementation. The list of new requirements:

1. I often want to know when I created the snippet. So, here is my request: record the creation time when creating the snippet.
2. Add a new storage option: [SQLite](https://sqlite.org/index.html) database.
   Your app should read the `SNIPPETS_APP_STORAGE` environment variable and use the storage provider depending on this environment variable content.
   The `SNIPPETS_APP_STORAGE` value should have the following pattern: `<storage provider name>:<file path>`. Your app must support two storage providers: `JSON` and `SQLITE`.
   Here are a few examples:
   | `SNIPPETS_APP_STORAGE` value example | meaning |
   |-|-|
   | `JSON:/home/pavlo/snippets.json` | The app should use the `/home/pavlo/snippets.json` file to store all code snippets. |
   | `SQLITE:/home/pavlo/snippets.sqlite` | The app should use the `/home/pavlo/snippets.sqlite` as the SQLite database file. |

Let me add more context to decrease the confusion :upside_down_face:. SQLite is a simple database that stores all your data in _**one file**_. It doesn’t need a server and works right inside your app. You can use it to save and read information using normal SQL commands.

There are a lot of query builders and ORMs in the Rust ecosystem. I recommend the [`rusqlite`](https://docs.rs/rusqlite) library.

## Self-learn

- https://github.com/rust-lang-ua/rustcamp/blob/master/1_concepts/1_6_dispatch/README.md
- https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility
- https://joshleeb.com/posts/rust-traits-and-trait-objects
- https://medium.com/digitalfrontiers/rust-dynamic-dispatching-deep-dive-236a5896e49b
