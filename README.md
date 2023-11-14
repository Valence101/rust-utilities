# rust-utilities

Learning rust: https://doc.rust-lang.org/book/ch01-01-installation.html  
- `rustup doc` - local documentation
- Default (windows) install location: `%userprofile%\.cargo\bin`

## Daily Ops

- `rustup update`

## Notes

### 1.2

- `rustc main.rs` - to compile
- `println!` - `!` indicates we are calling a macro, not a function

### 1.3

- `cargo` is the package manager & build tool
- `cargo new <project name>` - creates your boilerplate dir structure and hello world
- `cargo build` - use the `--release` argument to include optimizations
- `cargo run` - will also automatically build if src files changed
- `cargo check` - to compile without overwriting your exe (to check for compilation errors)

