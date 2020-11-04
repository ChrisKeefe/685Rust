draw a syntax tree of a struct
draw a syntax tree of a match arm
compare

read about the term glob

Do the summary exercises for collections: https://via.hypothes.is/https://doc.rust-lang.org/book/ch08-03-hash-maps.html

Try modifying Cacher to hold a hash map rather than a single value. The keys of the hash map will be the arg values that are passed in, and the values of the hash map will be the result of calling the closure on that key. Instead of looking at whether self.value directly has a Some or a None value, the value function will look up the arg in the hash map and return the value if it’s present. If it’s not present, the Cacher will call the closure and save the resulting value in the hash map associated with its arg value.
https://via.hypothes.is/https://doc.rust-lang.org/book/ch13-01-closures.html

After Ch13, revisit https://via.hypothes.is/https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html and look up `unwrap_or_else`.

Look at quiz to and the `into()` method

- refactor errors3.rs giving main() a return type
- ch 10.02, 3rd highlight mark. Trait return type exercise.
- ch 17.3 - work through trait objects and oop with "blog" code
