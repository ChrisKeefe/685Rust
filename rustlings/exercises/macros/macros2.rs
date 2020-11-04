// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

// In this solution, I've just re-ordered things. If the macro definition was in
// a distinct module, I think we'd need #[macro_use] on that module
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}

// Not entirely sure why this works, but it does!
//fn main() {
//    my_macro!();
//}
//
//#[macro_export]
//macro_rules! my_macro {
//    () => {
//        println!("Check out my macro!");
//    };
//}
//