// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

// I AM NOT DONE

// Initial solution - clone
// fn main() {
//     let vec0 = Vec::new();
// 
//     let mut vec1 = fill_vec(vec0.clone());
// 
//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
// 
//     vec1.push(88);
// 
//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }
// 
// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;
// 
//     vec.push(22);
//     vec.push(44);
//     vec.push(66);
// 
//     vec
// }

// Solution 2 - use borrowing
// fn main() {
//     let vec0 = Vec::new();
// 
//     let mut vec1 = fill_vec(&vec0);
// 
//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
// 
//     vec1.push(88);
// 
//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }
// 
// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.to_vec();
// 
//     vec.push(22);
//     vec.push(44);
//     vec.push(66);
// 
//     vec
// }

// Solution 3 - use mutability and remove vec1 entirely
fn main() {
    let mut vec0 = Vec::new();

    vec0 = fill_vec(vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
