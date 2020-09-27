// import the library under test first
use adder;

// Then get on with it as usual.
#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
