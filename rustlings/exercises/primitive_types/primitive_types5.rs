// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);

// Destructuring assignment also works with arrays, but _you must unpack into an array_.
    let cats = ["Furry McFurson", "Stinky Stinkereeno"];
    let [good_cat, bad_cat] = cats;
    println!("{} got beat up by {}.", good_cat, bad_cat);
}