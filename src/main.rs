// Implementation as function and variables for height, width. Could also use a tuple as function arg to tie together the 
// dimensions of a single rectangle.
//
// fn area(width: u32, height: u32) -> u32{
//     width * height
// }
// 
// fn main() {
//     let width1 = 30;
//     let height1 = 50;
// 
//     println!(
//         "The area of the rectangle is {} square pixels.", area(width1, height1)
//     );
// }

// Implementation using structs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // everything within this impl block will be associated with the Rectangle type, so can put all methods in here
    // although can have multiple impl blocks for a struct.
    
    fn area(&self) -> u32 { // method can either take ownership or borrow the instance. Self is an alias for the type of the impl block
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // we can also add associated functions (like the String::from() function) that don't have self and are not methods
    // they don't need an instance of the type to operate on, so there is no need for the self keyword in the parameters.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

}

// this is a standalone function 
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };
    
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.", rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let sq1 = Rectangle::square(3);

    println!("The area of the square is {} square pixels.", sq1.area());
}