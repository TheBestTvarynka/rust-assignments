
# Assignment 5: Configuring and logging.

## Part 1

Write a simple program that prints out its actual configuration. Configuration should be implemented as a typed hierarchical structure, which is able to parse from a specified file and/or environment variables.

The following priority should be applied (in ascending order) when merging:

1. Default values declared directly in Rust sources (lowest priority)
2. Values read from TOML file;
3. Values set by environment variables with CONF_ prefix. (highest priority)

CLI of the program should look like:

```
$ cargo run -- --help
Prints its configuration to STDOUT

Usage: task_3_9 [OPTIONS]

Options:
  -d, --debug        Enables debug mode
  -c, --conf <CONF>  Path to configuration file [env: CONF_FILE=]  [default: config.toml]
  -h, --help         Print help
  -V, --version      Print version
```

_Note 1: Place your implementation in the `./src/main.rs` file._

_Note 2: There is an example of the configuration file you app should be able to parse: [config.toml](./config.toml)._

## Part 2

Remember the `snippets-app` from the previous assignment? Good :smiling_imp:. In this part, you will add logging and proper configuring, and snippets downloading.

- Add logging to the app:
    - Use either the [`log`](https://docs.rs/log/) or [`tracing`](https://docs.rs/tracing) (**preferred**) crate.
    - Log level and log file should be configured using the `SNIPPETS_APP_LOG_LEVEL` and `SNIPPETS_APP_LOG_PATH` environment variables.
- Implement a new feature for the `snippets-app`: snippets downloading. Add a new CLI option `--download <URL>`. When this option is present, the app will download the snippet from the provided URL instead of reading from `stdin`.
  Example:
  ```bash
  ./snippets-app --name "minimalistic_tracing_logger" --download "https://gist.githubusercontent.com/TheBestTvarynka/bb2e8fee52abaf3bf1e9b567453d7466/raw/"
  ```
  The command above will create a "minimalistic_tracing_logger" snippet with the content behind this URL.
  You can use any HTTP client for request handling, but I recommend using the [`reqwest`](https://docs.rs/reqwest) crate (with the `blocking` feature on because your app is synchronous. Async Rust is out of the scope of the current course).
- If you did not use the [`clap`](https://docs.rs/clap) or any other crate for CLI args handling and did CLI args parsing manually, then rework args parsing using the [`clap`](https://docs.rs/clap) crate.

## Self-learn

- https://github.com/rust-lang-ua/rustcamp/blob/master/3_ecosystem/3_8_log/README.md
- https://docs.rs/tracing
- https://docs.rs/config/
- https://docs.rs/clap/
- https://docs.rs/reqwest/
