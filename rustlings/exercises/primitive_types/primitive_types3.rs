// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is. 
// Execute `rustlings hint primitive_types3` for hints!

fn main() {
    let oh_no = ['🐙'; 666];

    if oh_no.len() >= 100 {
        println!("Run for your lives!");
        for i in 0..oh_no.len() {
            println!("{}", oh_no[i]);
        }
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
