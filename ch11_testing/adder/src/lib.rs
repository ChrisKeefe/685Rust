#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller()
    {
        let larger = Rectangle{height:10, width:10};
        let smaller = Rectangle{height:5, width:5};
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle {width: 10, height:10};
        let smaller = Rectangle {width: 5, height:5};

        assert!(!smaller.can_hold(&larger));
    }
 
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
