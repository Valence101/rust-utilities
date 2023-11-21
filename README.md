# rust-utilities

Learning rust: https://doc.rust-lang.org/book/ch01-01-installation.html  
- `rustup doc` - local documentation
- `cargo doc --open` - local documentation that includes documentation for all the crates you happen to have included in your project dependencies listing
- Default (windows) install location: `%userprofile%\.cargo\bin`
  
Rust is a statically typed language (all variable data types must be known at compile time).  

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

### 3

Common Programming Concepts

#### 3.1 - Variables and Mutability

- `let` - to declare immutable variable
- `const` - to declare a constant, must have a data type annotation
  - unlike C#, constants can be declared like so: `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;` - where the compiler will evaluate the expression as part of the compilation, this way the source code is easier to understand (for a human) - but is ultimately compiled down to the constant we are used to seeing in other languages.s
- `let x = 5; let x = x + 1;` - this is called shadowing
  - this allows us to keep `x` as immutable, while performing transformation operations on it (thanks to the `let` keyword)
  - as part of this, you can also change the data type of the variable (very cool), e.g.:
  ```rust
  let spaces = "   ";
  let spaces = spaces.len();
  ```
  - shadowing can also be used with inner scopes and the values are maintained independent of the outer scope

#### 3.2 - Data Types

- variables fall into two categories:
  - Scalar
    - integers: `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `i128`, `u128`, `isize`, `usize`
    - floating-points: `f32`, `f64` (all are signed, f64 is the default for implicitly defined variables)
    - boolean: `let f: bool = false;` (`true` | `false`)
    - char: `let z: char = 'Z';` - 4 bytes in size, unicode scalar value
  - Compound
    - tuple: `let tup: (i32, f64, u8) = (500, 6.4, 1);` fixed in size, cannot grow or shrink after declaration
      - to read values out of a tuple, you use "destructuring": `let (x, y, z) = tup; println!("The value of y is: {y}");`
      - or use dot-notation followed by the index value: `let five_hundred = tup.0;`
    - array: `let a = [1, 3, 44, 34];` elements must be of the same type, fixed length
      - allocated to the stack, not heap
      - strongly typed: `let a: [i32; 5] = [1, 3, 44, 34, 44];`
      - initialized with a default value of '3': `let a: [3; 5];`
