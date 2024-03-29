# Rust Axum library

## new project

```bash
mkdir rust-library
cd rust-library
git config --global init.defaultBranch main
git init
touch Cargo.toml
```

Cargo.toml

```toml
[workspace]

members = [
    "rs_library",
]
```

use **cargo --list** to find more command

```bash
cargo new rs_library --bin
```

[It has also initialized a new Git repository along with a .gitignore file. Git files won’t be generated if you run cargo new within an existing Git repository](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

refer [new project](https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html)

```toml
[workspace]
resolver = "2"

members = [
    "rs_library",
    "rs_system",
]

```

```bash
cargo new rs_system --lib
```

library/Cargo.toml

```toml
[dependencies]
system = {path="../rs_system"}
```

library/src/main.rs

```rust
use system;

fn main() {
    let num = rs_system::add(1, 2);
    println!("Hello, world!{num}");
}
```

build and run

```bash
cargo build
cargo run -p rs_library
```

### auto-reload

[auto-reload](https://github.com/tokio-rs/axum/blob/main/examples/auto-reload/README.md)

## Profiles

Cargo.toml

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

The opt-level setting controls the number of optimizations Rust will apply to your code, with a range of 0 to 3.

refer [Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
