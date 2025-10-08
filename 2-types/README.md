
# Assignment 2: Types.

This task aims to give you an experience of improving software correctness by covering data with types.

This practical assignment consists of three parts.
Parts 1 and 2: solving training problems.
Part 3: implementing your own application (using new techniques you learned, of course).

## Part 1

For the `Post` type described above, assume the following behavior in our application:

```
+-----+              +-------------+            +-----------+
| New |--publish()-->| Unmoderated |--allow()-->| Published |
+-----+              +-------------+            +-----------+
                           |                          |
                         deny()                    delete()
                           |       +---------+        |
                           +------>| Deleted |<-------+
                                   +---------+
```

Implement this behavior using [typestates idiom](https://yoric.github.io/post/rust-typestate), so that calling `delete()` on `New` post (or calling `deny()` on `Deleted` post) will be a compile-time error.
**Write simple tests for the task.**

_Place your implementation in the `./part_1-2/src/part_1.rs` file._

## Part 2

Write a program which deserializes the [following JSON](request.json) into a static `Request` type and prints out its serialization in a TOML format.
Consider choosing the correct types for data representation.

**Prove your implementation correctness with tests.**

_Place your implementation in the `./part_1-2/src/part_2.rs` file._

## Part 3

Do you know this feeling when you are working with many large codebases and you often want to store small code snippets somewhere else and reuse them later? It can be repetitive code, a code pattern you want to reuse, or just some code you do not want to forget...

In the scope of this task, you will implement a simple CLI app for storing and listing code snippets. Functionality description:

* Snippets creation by reading snippet data from `stdin`. Accept snippet name as CLI argument. Example:
  ```bash
  # Create code snippet
  echo "if let Some(local_time) = self.local_time else { }" | ./snippets-app --name "Cool Rust pattern"
  ```
* Snippet reading by name. Accept snippet name as CLI argument. Example:
  ```bash
  # Create code snippet
  echo "if let Some(local_time) = self.local_time else { }" | ./snippets-app --name "Cool Rust pattern"
  # Read code snippet
  ./snippets-app --read "Cool Rust pattern"
  # The command above should print:
  # if let Some(local_time) = self.local_time else { }
  ```
* Snippet deletion by name. Accept snippet name as CLI argument. Example:
  ```bash
  # Delete code snippet
  ./snippets-app --delete "Cool Rust pattern"
  ```

1. Use the simplest approach possible. Do not overcomplicate the solution.
2. Store snippets in any way you found suitable. It can be a single `.json`/`.txt` file. As you want.
3. You are allowed to use external dependencies, but I recommend keeping it simple.
4. Be aware that you will work with this app in your future assignments. So, it is in your interest to write clean code :)

_Place your implementation in the `../snippets-app` crate._

## Self-learn

### New-type pattern

- https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html
- https://fettblog.eu/refactoring-rust-abstraction-newtype/

### Type-state pattern

- https://yoric.github.io/post/rust-typestate/
- https://cliffle.com/blog/rust-typestate
- https://www.greyblake.com/blog/builder-with-typestate-in-rust/
