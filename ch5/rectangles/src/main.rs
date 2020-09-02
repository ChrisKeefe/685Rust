// Using Method Syntax
fn main() {
    #[derive(Debug)]
    struct Rectangle {
        length: u32,
        width: u32
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.length * self.width
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
        }
    }

    let rect1 = Rectangle{
        length: 30,
        width: 50
    };

    let rect2 = Rectangle{
        length: 10,
        width: 40
    };

    // {:?} is simple Debug print. {:#?} is pretty
    println!("{:?}", rect1);

    println!("Area: {} square pixels", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2))
}

// With Derived Traits
// fn main() {
//     #[derive(Debug)]
//     struct Rectangle {
//         length: u32, 
//         width: u32
//     }
// 
//     let rect1 = Rectangle{
//         length: 30,
//         width: 50
//     };
// 
//     // {:?} is simple Debug print. {:#?} is pretty
//     println!("{:?}", rect1);
//     println!("Area: {} square pixels", area(&rect1));
// 
//     fn area(rectangle: &Rectangle) -> u32 {
//         rectangle.length * rectangle.width
//     }
// }

// With Structs
// fn main() {
//     struct Rectangle {
//         length: u32, 
//         width: u32
//     }
// 
//     let rect1 = Rectangle{
//         length: 30,
//         width: 50
//     };
// 
//     println!("Area: {} square pixels", area(&rect1));
// 
//     // Remember! Borrowing here allows main to keep ownership
//     fn area(rectangle: &Rectangle) -> u32 {
//         rectangle.length * rectangle.width
//     }
// }

// // With Tuples
// fn main() {
//     let rect1 = (30, 50);
// 
//     println!("Area: {} square pixels", area(rect1));
// 
//     fn area(dimensions: (u32, u32)) -> u32 {
//         dimensions.0 * dimensions.1
//     }
// }

// First Pass
// fn main() {
//     let width1 = 30;
//     let height1 = 50;
// 
//     println!("Area: {} square pixels", area(width1, height1));
// 
//     fn area(width: u32, height: u32) -> u32 {
//         width * height
//     }
// }
// 