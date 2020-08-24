// functions1.rs
// Make me compile! Execute `rustlings hint functions1` for hints :)

// NOTE: Though the hint suggests the compiler expects `call_me()` not to
// return a value, the test passes happily with a return, even if it's unused

// fn main() {
//     println!("{}", call_me());
// }

fn main() {
    call_me();
}

fn call_me() -> String {
    return "maybe?".to_string();
}