// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

fn main() {
    // Interesting! You can re-declare an immutable variable. Is this shadowing?
    // Or is it actually reassigning?
    let x = 6;
    let x = 10;
    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}
