# rust-utilities

Learning rust: https://doc.rust-lang.org/book/ch01-01-installation.html  
- `rustup doc` - local documentation
- `cargo doc --open` - local documentation that includes documentation for all the crates you happen to have included in your project dependencies listing
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

### 2

- `use std::io;` - library import
- `let` - declare immutable variable
- `let mut` - declare mutable variable
- `::` - allows traversing library > class/type > function
- `&` indicates "by reference" for input argument - e.g. `io::stdin().read_line(&mut guess);`
- [crates.io](https://crates.io/) - public package (crate) manager - like nuget.org or npmjs.com
- `cargo update` - updates packages to latest bug fix (based upon semantic versioning)
  - you will need to update the `Cargo.toml` file to explicitly upgrade to new feature versions or breaking versions
- [TODO] Generating a secret number