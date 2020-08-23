# Chapter 1: Hello World

## General

- fn main() is the universal entry-point for rust programs.
- semicolons indicate the end of an expression, as usual. Yay!
- `!` indicates a _macro_ is being called, not a function
- 'binary crates' produce executables, while 'library crates' prodce ... libraries
- Rust has no concept of Truthiness: conditions must evaluate to a bool
- Rust has Pythonic ternaries: `let x = if condition { 5 } else { 6 };`

## Style

Rust fmt prefers:

- "one true brace"
- four space indents
- constants should be all-caps
- function names should be in snake case
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

- all parts of a function's signature must have type annotations
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
  - variables are immutable by default
  - so are references! (`& mut foo` is a thing)
- `const x: u32 = 5`: const creates a constant
  - values assigned to constants must be fixed at compile time.
  - Types _must_ be annotated when using `const`
  - function calls can't be used in initializing consts, unless they are const functions: `const fn get_value():`

## Data types

### Scalar types

- integers
  - u<bits> or i<bits>, where bits can be powers of 2 from 8 - 128
  - `usize` and `isize` vary based on 32-bit vs 64-bit architectures
  - integer literals may be type-suffixed. e.g. `57u8`
  - integer literals may also have `_` visual separators: `1_000` = 1,000
  - i32 is generally the fastest type, even on 64-bit architectures
- floats
  - f32 or f64: defaults to f64, because same speed, more precision on modern CPUs
- Booleans
  - `bool` type can be `true`, `false`
- characters
  - `char` type is 4 byte Unicode Scalar Value

### Primitive Compound Types

- tuples
  - fixed-length
  - fmt: comma-separated values in parens: `let tup: (i32, f64, u8) = (-1, 7.0, 255);`
  - each position has a type; these may be heterogenous. type annotations optional
  - pattern matching (destructuring assignment) works: `let (x, y, z) = tup;`
  - so does indexed access: `let neg_one = tup.0`
- arrays
  - fixed-length (but vectors can grow/shrink)
  - fmt: comma-separated values in brackets: `let arr: [i8; 3] = [-1, 7, 255];`
  - one type per array, indicated in brackets
  - second number in optional type annotation is number of elements
  - arrays are a "single chunk of memory on the stack"
  - create an array full of the same element: `let arr = [0; 3];` equiv to `let arr = [0, 0, 0];`
  - access using indexing: `let neg_one = arr[0];`
  - slice like so: `let all_inclusive_slice = &some_array[0..some_array.len()];`
  - Rust catches array overrun errors at runtime with "index out of bounds"

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

## syntax, operators, etc

- "The underscore, `_`, is a catchall value; `Err(_)` matches all `Err` values"
- underscores can also be used as visual separators in ints
- +, -, *, /, % : integer division not mentioned in the book
- char looks like 'a', string looks like "a"

## If

- no parens required around if conditions `if x>5 {}'
- `else if` combines if with else

## Loops
- `loop` executes until explicitly halted, like a dedicated `while(True)`
  - `break` is used to break the loop, and can "return" the result of an expression. If we assign the whole loop to a variable, `break counter` return the value of `counter` and save it to the variable
  - useful in retrying operations that might fail
  - Emphasizes that this loop continues indefinitely, unless some event occurs.
- `while` - standard while loop. condition does not require parens. Emphasizes that loop occurs "while a condition remains true"
- `for` 

## Questions

- must the arms of a `match` statement be mutually exclusive? Can `match` match more than one arm?
  - I suspect it must be mutually exclusive: "The match expression ends [short-circuits] because it has no need to look at the last arm [if it has already found a match]"
