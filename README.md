# easy-envar

[![Github](https://img.shields.io/badge/github-foolkat/easy_envar-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/foolkat/rust-easy-envar)
[![Crates.io](https://img.shields.io/crates/v/easy-envar.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/easy_envar)
[![Docs.rs](https://img.shields.io/badge/docs.rs-easy_envar-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/easy_envar)
[![Build](https://img.shields.io/github/actions/workflow/status/foolkat/rust-easy-envar/rust.yml?branch=main&style=for-the-badge)](https://github.com/foolkat/rust-easy-envar/actions?query=branch%3Amain)

Easily retrieve, parse, and export environment variables directly within your build script (`build.rs`).
By loading from an `.env` file and exporting each variable through Cargo, you can reliably access the same values at compile time in your application code.

---

## Examples

```rust
// build.rs

use easy_envar::Envar;

fn main() {
    // Load the .env file
    if easy_envar::init().is_err() {
        eprintln!("environment file not found!");
        std::process::exit(1);
    }

    // Define environment variables
    let env_vars = [
        Envar::String("HOST"),
        Envar::U16("PORT"),
        Envar::Bool("USE_SECURE"),
    ];

    // Load and export environment variables
    for env_var in env_vars {
        match env_var.load() {
            Ok(env_var) => env_var.export(),
            Err(e) => {
                eprintln!("Failed to load an environment variable: {}", e);
                std::process::exit(1);
            }
        }
    }
}
```

```rust
// main.rs

fn main() {
    // Because 'build.rs' already loaded and exported these values,
    // the following calls won't fail as long as the .env file is valid.
    let host: String = std::env!("HOST");
    let port: u16 = std::env!("PORT").parse().unwrap();
    let use_secure: bool = std::env!("USE_SECURE").parse().unwrap();

    println!("Host: {}", host);
    println!("Port: {}", port);
    println!("Secure: {}", use_secure);
}
```

---

## Error Handling

- **Missing `.env` file**:  
  If the `.env` file is not found, `init()` returns an error. In the example, the script prints a warning and exits.

- **Missing environment variable**:  
  If a defined variable is not present in the `.env` file (or system environment), `load()` returns an error.

- **Parsing errors**:  
  When calling `load()`, the library attempts to parse the string into the specified type (`String`, `bool`, or `u16`). If parsing fails (e.g., `PORT=not_a_number`), an error is returned.

---

## Why Use `easy_envar`?

- **Compile-Time Guarantees**:  
  By exporting environment variables through Cargo, you can use `env!` in your code to ensure the variable is present at compile time, avoiding runtime surprises.

- **Type Safety**:  
  `easy_envar` enforces type parsing in your `build.rs`. If something doesn’t parse (e.g., a `bool` is `TRUEE`), you’ll know early.

- **Simple Integration**:  
  The API is concise and straightforward, leveraging `dotenvy` for file loading and standard Rust `.parse()` methods for parsing.
