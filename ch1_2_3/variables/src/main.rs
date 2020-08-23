// constants can't be initialized with function calls unless they're const fn's
const FIVE: u32 = get_five();
const fn get_five() -> u32 {
    let y = 2+3;
    return y;
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("{}", FIVE);
}

