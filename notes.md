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
- the Rust book, at least, leaves trailing commas on all lines in e.g. a struct

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
- Separation of Concerns in main: place all program logic in lib.rs. `main.rs::main` holds only:
  - call command line parsing logic with arg values (or run that logic locally if very small)
  - set up configuration
  - call a `run` function in `lib.rs`
  - handle any errors from `run`

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

## Primitive Data types

- primitive types are stored on the stack while in scope
- here, `let x = 5; let y = x;`, the _value_ `5` is copied and assigned to `y`
  - this behavior is driven by the `Copy` trait, and is applicable to:
    - All int types
    - All float types
    - bool
    - char
    - Tuples, if they exclusively contain types that are also `Copy`

### Scalar types

- integers
  - `u<bits>` or `i<bits>`, where bits can be powers of 2 from 8 - 128
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
  - fixed-length (vectors OTOH can grow/shrink)
  - fmt: comma-separated values in brackets: `let arr: [i8; 3] = [-1, 7, 255];`
  - one type per array, indicated in brackets
  - second number in optional type annotation is number of elements
  - arrays are a "single chunk of memory on the stack"
  - create an array full of the same element: `let arr = [0; 3];` equiv to `let arr = [0, 0, 0];`
  - access using indexing: `let neg_one = arr[0];`
  - slice like so: `let all_inclusive_slice = &some_array[0..some_array.len()];`
  - Rust catches array overrun errors at runtime with "index out of bounds"

## Complex Data types

- Stored on the heap, subject to ownership rules
- `String`, for example, is _literally_ a struct on the stack with a pointer to heap memory and length/capacity values
- In `let x = String::from("hello"); let y = x;`, the _struct_ is copied and assigned to `y`, giving `y` a pointer to the value `hello` in memory. When this happens, x is _invalidated_, its values may not be borrowed, and it will not be freed (protecting us from double-free-ing the data). Rather than a `shallow copy`, then, this is called a `move`.

### Slice type

- immutable
- slices store a pointer to the starting position and the length of the slice
- array slices are of type `&[<type_of_internal_data>]`
- string slices are signified by type annotation: &str
- string literals are actually slices pointing to a specific point in the binary
- prefer string slices for fn parameters: you can pass them a slice, or a full-length slice of a String

#### Range syntax

[0..2] == [0, 1]
[..2]  == [0, 1]

[3..s.len()] == [3..]

[..] == [0..s.len()] -> take a slice of the entire string

### String literals

- immutable, must be declared at compile time
- stored on the stack
- These may actually be primitive compounds? Not sure.
- indexing into strings is not supported. slicing strings is supported but dangerous.
- prefer `for c in "mystring".chars() {}` for accessing characters, and `.bytes()` for bytes. Grapheme access is provided by crates outside of std lib.

### String

- stored on the heap, memory is requested at runtime and must be returned
- can be created from literals with:
  - `let s = String::from("some literal");`
  - `let s = "some literal".to_string();`
- mutable, e.g.: `s.push_str(" appended text");`
- like a Vec, and unlike a literal, `String` is _actually_ a struct holding a pointer, length and capacity values
- the `String` _struct_ lives on the stack, but points to an address in heap memory
- grow Strings with:
  - `+` adds a String and a &str, dereferencing and taking/returning ownership of the String in the process. Not recommended for more than two addends
  - `format!` macro returns a string just like `println!`: `let s = format("{} is a {}", s1, s2);` Takes actual Strings happily.
  - `mystring.push_str("other string");` (or `.push()` to add a single char literal)

### Vector type

- contiguous, growable, homogenous array type
- Vec homogeneity can be hacked by defining an enum with different types, then creating a `Vec<MyEnum>` full of `MyEnum::TypeIActuallyWant("gerbil")`. Also possible to hack this with Trait objects
- under the hood, Vec is a (pointer, capacity, length) triplet
- implements Index - values are ordered and indexed from 0
- implemented with generics; when initializing `Vec::new();`, we must specify a type: `let v: Vec<i32> = Vec::new()`
- more often, we use the `vec!` macro to declare and initialize with values: `let v = vec![1, 2, 3]` allows rust to infer the default integer type (`i32`)
- methods include:
  - `push`
  - `pop`
  - `len` (checks size, not capacity)
  - `append` empties another Vec into `self`
  - `clear`
  - `is_empty`
  - `split_off` breaks Vec into two Vecs at index
  - `remove` drops a value by index and shifts remaining vals left
  - `retain` drops elements where `expression == false`, retaining the rest in order
  - `dedup_by*`
  - `truncate`
  - `shrink_to_fit`
  - `as_mut_slice`
- when values overflow capacity, all values must be reallocated (which can be slow). Use `Vec::with_capacity` when possible to specify max capacity

#### Accessing Vector Values

- `&vecname[]` syntax returns a reference
  - e.g. `let third: &u32 = &v[2];`
  - causes panic if index OOB
- `vecname.get(2)` returns an `Option<&t>`
  - e.g.```match v.get(2) {
             Some(third) => println!("The third element is {}", third),
             None => println!("There is no third element."),
           }```
  - `match` handles the OOB intelligently
  - standard mutability and ownership rules apply
  - ... so an immutable reference to a value in `myvec` may not exist when we `push` or `pop`
  - iterate over a vec with `for i in &myvec {}` or `for i in &mut myvec {}`

## Hash Maps

- `HashMap<K, V>` is _not accessible in prelude_. `use std::collections::HashMap;`
- K and V are homogenous types
- Unordered.
- `HashMap::new()` then `mymap.insert(String::from("foo"), 5);`
- alternately, use `zip(...).collect()` to make `(K, V)` tuples, then collect them into a `mut mymap: HashMap<_, _>`
- owned types passed into a Hash Map with `.insert()` will be moved, so no longer accessible in the parent scope. References can be used to circumvent this (with lifetimes). Types that implement `Copy` will be copied.

### Accessing Hash Maps

- `mymap.get()` returns an `Option<&V>`
- insert with overwrite on key collision: `mymap.insert()`
- insert if key does not exist: `mymap.entry(String::from("foo")).or_insert(50);`

## Error Handling

- many error-prone functions return `Result`
- handle results with `match`, `unwrap`, `expect`, etc.
- we can propagate errors up to calling code for handling by `return`ing them explicitly. The `?` operator does this more succinctly.
- This can only be done in functions that return `Result`, `Option`, or other objects that implement `Try`, unless we handle the result within the function using `match` or similar.
- Ahen trying to assign a value wrapped in a Result which may error, use `unwrap_or_else` or similar. Alternately, use `if let...` if you don't have a `Result`-wrapped value you need to unwrap. See Ch12-03 for details.

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
- `cargo test` tells rust to build a test-runner binary, and run all annotated functions
  - It is possible to pass args to the test runner or the resulting binary, using the `--` separator: `cargo test --args_for_test_runner -- --args for binary`
  - See these options with `cargo test --help` and `cargo test -- --help`

## Crates

- crates must have unique name, defined in `Cargo.toml` under `[package]` `name = "my_crate_name"`
- A license is required, description may be required too?
- Published crates are _permanent_! This protects software that uses your crate as a dependency.
- _The Book_ mentions using semVer. Is this required? If so, cool?
- `cargo yank` will prevent new projects from depending on a version of your project. Existing dependencies will still be supported.

## Workspaces

- a set of packages that share one `Cargo.lock` and output directory.
- Create a `Cargo.toml` with a `[workspace]` header that lists member packages: `members = ["package1", "package2"]`
- `cargo new package1` etc to create the packages in our workspace

## Documentation

- `rustup doc` builds and opens Rust documentation locally
- `cargo doc --open` builds and opens documentation locally for all crate dependencies

## Testing

- A test is just a function annotated with `#[test]`
- `cargo test` tells rust to build a test-runner binary, and run all annotated functions
- `assert_eq!` and `assert_ne!` use debug formatting and equality test operators. Values we pass them must implement `PartialEq` and `Debug` traits. These are both derivable traits, so #[derive(PartialEq, Debug)] will usually do the job.
- These macros (and `assert!`) take var args, so we can pass any number of values to them after the required. These are passed to `format!`, so including a format string with `{}`, followed by a variable allows us to report values programatically.
- we can assert_almost_equals by taking the difference and checking it is less than some threshold value
- the `should_panic` attribute is placed after the `test` attribute to pass tests that panic. The `expected` parameter allows us to specify an error string a la `assertRaises`. E.g. `#[should_panic(expected="some error message")]`. This string must be _contained by_ our actual error message, but doesn't need to cover the whole message.
- It's possible to return a `Result` from a test function (don't use `should_panic`), making it possible to use `?` and track multiple different error points in code under test.
- Defaults:
  - tests are run in parallel and _should not rely on any shared state_`
  - the test runner captures output from passing tests to keep test results clean (disable with `cargo test -- --show-output`)
  - all tests are run
- Code in a module under `#[cfg(test)]` will only be compiled during testing, and not during `cargo build` or `cargo run`
- Passing a filter string (`cargo test myfilter`) will run all tests with `myfilter` in their name
- Annotating slow tests with `#[ignore]` after `#[test]` will skip them unless `cargo test -- --ignored` is called

### Integration testing

- Place integration test files in a `tests` directory at the same level as `src`. This directory gets special treatment by Cargo. E.g.
  - Nothing in this directory is compiled unless we're using `cargo test`
- Each test file is treated as an individual crate, and an arbitrary number of these may exist.
- GOTCHA: The behavior described above breaks the pattern used in all other directories, impacting how we factor utility code out into shared files.
  - those files will be compiled and run as _test_ files by default
  - if we place common utilities in a _subdirectory_ of `tests` (e.g. `tests/common/mod.rs`), they don't get compiled as crates or run as integration test sections.
  - we can then use `mod common;` to use our utils from integration tests files. (see book 11.3 and 7.21)
- Use `cargo test --test integration_test_file` to run only the tests in one file

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
- `for` iterates over a collection of items

## Ownership/Borrowing

### The rules

- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Nuts and bolts

- `drop` is called when a variable goes out of scope, and frees its memory
- No deep copying by default in Rust. Use `x.clone()` if you need a deep copy.
- In `let x = String::from("hello"); let y = x;`, the data is `moved` not copied, leaving `x` invalid
- the `Copy` trait is used to indicate that values of a type should remain accessible after assignment to a second variable. Put differently, `Copy` objects are not `moved`.
- When a variable is passed to a function, it will be moved or copied as appropriate. E.g. a String passed to a function will no longer be valid in its declared scope, having been moved into the function scope. This invalidation does not affect `Copy` variables (e.g. a variable holding a `u32`)
- The same behavior is used by `return`

### References

- `&` reference operator
- `*` dereference operator
- references are immutable by default, but you can create one `mut & myvar` per variable
- curly braces `{}` can be used to create new scopes, in which we can make additional mutable references to a variable. This is possible, because they are creating new scopes, so there are never multiple _simultaneous_ mutable references
- it is not possible to have both mutable and immutable references to a single value at the same time...
- meaning that this is OK:
  
  ``` Rust
  fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
  }
  ```

- Above, we see that reference scope lasts only until the final _use_ of the variable, not until the end of the parent stack frame

#### Reference Rules

- At any given time, you may have _either_ one mutable reference _or_ any number of immutable references
- References must always be valid. (No dangling references to values that have gone out of scope)

## Generics

- available on:
  - structs:

    ``` Rust
      struct Point<T, U> {
        x: T,
        y: U,
      }
    ```

  - enums:

    ``` Rust
      enum Option<T> {
        Some(T),
        None,
      }
    ```

  - functions: `fn largest<T>(list: &[T])`
  - method:

    ``` Rust
    simpl<T> Point<T> {
      fn x(&self) -> &T {
          &self.x
      }
    ```

  - must be declared as generic prior to use. E.g. `impl<T> Point<T>`. The compiler knows `Point<T>` is generic only because we have `<T>` after `impl` first.

## Traits

### Defining Traits

``` Rust
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

- we can implement a trait on a type iff either the trait or type is local to our crate
- define a trait with `pub trait Summary { <method signature here>; }
- if we also define the function body, it is a default implementation.

### Using traits

`impl Trait` syntax (syntactic sugar to shorten trait bound syntax):

``` Rust
pub fn myFunc(item: &impl MyTrait){
  item.doMyTraitMethod()
}
```

Trait bound syntax:

``` Rust
pub fn myFunc<T: MyTrait>(item: &T){
  item.doMyTraitMethod()
}
```

`where` clause syntax (for complex trait bounds):

``` Rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

- More complex trait bound examples here: <https://via.hypothes.is/https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax>
- `&impl MyTrait` can take the place of a concrete type in a function signature, constraining which types that function can take as input
- `+` between Trait names requires both traits be implemented for that type. (Valid types are the intersection of Trait1 and Trait2 implementers)
- You can return "generic" types by requiring your return type implement SomeTrait: `fn returns_summarizable() -> impl Summary {...}`
- "Blanket implementations" implement a trait on all types that satisfy some trait bounds, enabling quick implementations on any relevant types: `impl<T: Display> ToString for T {...}`

## Lifetimes

- `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {...}` All of `x`, `y`, and the returned `str` must last as long as lifetime `'a`
- "we’re not changing the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints."
- lifetimes need only be used on parameters that may be returned from the function scope. I think?
- the lifetime of return vals must be related to the lifetime of function parameters. If we need to return a variable assigned within function scope, it's best to return it as an owned type, so the calling function can clean up.

## Packaging

- `pub` makes traits, structs, maybe other things? accessible for `use` outside the defining module
- sibling functions, modules, etc. can refer to other sibling modules, functions, etc, even if nonpublic
- In the case above, children of the nonpublic module must still be marked `pub` to be accessible.
- `super::somefunc()` allows relative-path access to objects in the parent scope
- structs may be marked `pub`, but their fields are still private by default
- enum variants, on the other hand, are public if the enum is public
- Prefer complete module paths in your code, unless you think it likely you will move groups of namespaced stuff together. In that case, a relative path means you can update calls in fewer places after moving the group.
- When `use`ing a function path, only specify the path to the parent module. This way, function calls make clear the function isn't locally defined. When `use`ing structs, enums, etc, we can specify the full path unless we have more than one type with the same name.
- Use external packages by adding a dependency to `Cargo.toml`, and `use`ing the items we want in our code.
- Nested paths allow us to use many parts of a package cleanly: `use std::{cmp::Ordering, io};`
- "glob" `*` allows us to use all public items defined by a path. `use std::collections::*;` Probably better to keep the parent namespace.
- "Using a semicolon after mod front_of_house rather than using a block tells Rust to load the contents of the module from another file with the same name as the module.": `mod front_of_house;`

## Iterators

- `iter()` method returns each element in a collection
