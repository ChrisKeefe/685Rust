// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// I AM NOT DONE

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    if maybe_number.is_some(){
        println!("printing: {}", maybe_number.unwrap());
    }
}

// I'm fundamentally not grasping the point of this exercise. Is there a reason
// we would ever wrap integers in Some() like this for printing that I'm missing? 

fn main() {
    print_number(Some(13));
    print_number(Some(99));


// Does using an array of Option<u16>s just make it possible for us to initialize
// the array to None so we can assign as needed later, checking manually for None?
// Maybe this is an optimization strategy, so we can avoid Vec's overhead? 
// And is this actually one exercise, or two?
    let mut numbers: [Option<u16>; 5];
    numbers = [None, None, None, None, None];
    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 1235) + 2) / (4 * 16)
        };

        numbers[iter as usize] = Some(number_to_add);
    }
}
