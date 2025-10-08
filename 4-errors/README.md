
# Assignment 4: Error handling.

## Task

Remember the `snippets-app` from the previous assignment? Good :smiling_imp:. In this part, you will implement proper error handling. The list of new requirements:

- Refactor the app implementation by implementing the proper error handling.
- I recommend using either [`thiserror`](https://docs.rs/thiserror/) or [`anyhow`](https://docs.rs/anyhow/) crate.
- Make errors informative and attach more context to the error.
- Do not panic (do not use `.unwrap()`/`.expect()`/`panic!()`)!

## Self-learn

### `thiserror`

- https://docs.rs/thiserror/
- https://doc.rust-lang.org/book/ch09-00-error-handling.html
- https://dev.to/sgchris/how-to-handle-errors-gracefully-with-thiserror-2h56
- https://medium.com/rustaceans/a-comprehensive-guide-to-robust-code-with-thiserror-for-rust-43778b1b3906

### `anyhow`

- https://docs.rs/anyhow/
- https://leapcell.medium.com/simplifying-rust-error-handling-with-anyhow-0ec80474e333
