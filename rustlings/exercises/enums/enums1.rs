// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    // NOTE: it appears to be impossible to give these variants types, as
    // Debug is implemented on enum _values_, not constructors. (These are apparently
    // functions) under the hood: 
    // https://stackoverflow.com/questions/53927598/debug-trait-not-implemented-for-some-enum-variants
    Quit,
    Echo,
    Move,
    ChangeColor
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
