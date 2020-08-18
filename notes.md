# Chapter 1: Hello World

## General

- fn main() is the universal entry-point for rust programs.
- semicolons indicate the end of an expression, as usual. Yay!
- `!` indicates a _macro_ is being called, not a function
- 'binary crates' produce executables, while 'library crates' prodce ... libraries

## Style

Rust fmt prefers:

- "one true brace"
- four spaces. Actual yay!
- package names should be in snake case, and _may not_ include spaces
- whitespace and newlines when method chaining!

  ``` Rust
  // NO
  some_constructor().some_method().expect("you broke it");
  // YES
  some_constructor()
     .some_method()
     .expect("you broke it");
  ```

## coding patterns

- function mutates a passed argument, returns a Result (e.g. OK/ERR code)
  - e.g. `std::io::stdin().read_line(&mut some_string);`

## Comments

- `//` comments continue until EOL

## Strings

- create a new, empty string instance with `String::new()`
- Strings are growable, UTF-8-encoded text

## Functions

- `<Type>::<associated_function>()` calls an associated function (implemented on a type, not an instance - like a static method)
- similarly, `<Module>::<function>` calls a function from a module
  - The following are equivalent:

    ``` rust
    use std::io
    // ...
    io::stdin()
    ```

    ``` rust
    // ...
    std::io::stdin()
    ```

  - `.read_line(&spam)` calls the read_line method on the standard input handle

## Variables

- `let foo = bar`: `let` creates a variable
- `let mut bar = baz`: `mut` makes bar a mutable variable
- variables are immutable by default (so are references! `& mut foo` is a thing)

## Cargo

- `cargo new <program_name>` initiates new project
  - This includes a git repo (unless in one already)
  - Repo initialization can be disabled with `--vcs none` flag`
  - generates a `hello_world` in `main.rs` by default (disable this?)
  - possible to generate a `lib.rs` instead of a `main.rs` with `--lib` flag
- `cargo build` from project root compiles (and links?) your project
  - also creates a `Cargo.lock`
  - executables stored in `/target/debug`
  - `--release` flag optimizes code, at the expense of increased compile time
- `cargo run` runs `cargo build` and then runs the compiled executable
  - `--release` can also be used here
- `cargo check` checks for compile-ability without producing an executable
  - significantly faster for larger projects. good for interim compile checks
- `cargo fix` has the ability to lint and/or update legacy projects to current edition
- `cargo clean` removes the `target` directory
  - NOTE: working in VSCode, with Rust and rust-analyzer extensions enabled, target regenerates after deletion.
    Q: Should this be adusted in `.gitignore`?
    A: Yes, unless committing executables is preferred for some reason. And it is, in the standard github .gitignore for Rust

## Documentation

- `rustup doc` builds and opens Rust documentation locally
- `cargo doc --open` builds and opens documentation locally for all crate dependencies

## Std. Lib

- `cmp` is called on a comparable value, and passed a _reference_ to a value to which it can be compared
  - `value.cmp(&other_value)`
  - returns an Ordering enum, which we can check and respond to using `match`
- `parse` parses a string slice into another type, returning a Result (Ok/Err)

## Questions

- must the arms of a `match` statement be mutually exclusive? Can `match` match more than one arm?
  - I suspect it must be mutually exclusive: "The match expression ends [short-circuits] because it has no need to look at the last arm [if it has already found a match]"

## syntax, operators, etc

- "The underscore, `_`, is a catchall value; `Err(_)` matches all `Err` values
