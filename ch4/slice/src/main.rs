fn main() {
    // Take a string
    let s_lit = "Hello world!";
    let s = String:: from(s_lit);
    // split and print
    let hello_from_string = first_word(&s[..]);
    println!("{}", hello_from_string);

    let hello_from_slice_of_literal = first_word(&s_lit[..]);
    println!("{}", hello_from_slice_of_literal);
    
    let slice_actually_is_a_literal = first_word(s_lit);
    println!("{}", slice_actually_is_a_literal);

    let hello_from_dereferenced_string = first_word(&s);
    println!("{}", hello_from_dereferenced_string);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // enumerate returns a tuple with (index, _reference_ to value)
    for (i, &byte) in bytes.iter().enumerate() {
        // b' ' is the byte literal syntax for the ' ' character
        if byte == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Initial, lesser approach:
// Return the index of the first space in a &string
// This approach is suboptimal, because it separates two tightly coupled values:
// The string, and the index of its first space. 
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
// 
//     // enumerate returns a tuple with (index, _reference_ to value)
//     for (i, &byte) in bytes.iter().enumerate() {
//         // b' ' is the byte literal syntax for the ' ' character
//         if byte == b' ' {
//             return i;
//         }
//     }
// 
//     s.len()
// }