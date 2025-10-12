
# Assignment 7: Code formatting, lints, documentation.

## Task

- Format the `snippets-app` code using `cargo fmt`.
- Write doc comments for all _public_ items in the codebase.
- Enable the following Rust lints and fix warnings (run `cargo build` and you will see):
    - [`missing_docs`](https://doc.rust-lang.org/rustdoc/lints.html#missing_docs)
    - [`broken_intra_doc_links`](https://doc.rust-lang.org/rustdoc/lints.html#broken_intra_doc_links)
    - [`missing_crate_level_docs`](https://doc.rust-lang.org/rustdoc/lints.html#missing_crate_level_docs)
    - `unreachable_pub`
- Enable the following `clippy` lints for the entire codebase and fix clippy warnings (run `cargo clippy` and you will see):
    - [`missing_panics_doc`](https://rust-lang.github.io/rust-clippy/stable/index.html#missing_panics_doc)
    - [`clone_on_ref_ptr`](https://rust-lang.github.io/rust-clippy/stable/index.html#clone_on_ref_ptr)
    - [`similar_names`](https://rust-lang.github.io/rust-clippy/stable/index.html#similar_names)

## Self-learn

- https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html
- https://doc.rust-lang.org/stable/clippy/usage.html
- https://rust-lang.github.io/rustfmt/
