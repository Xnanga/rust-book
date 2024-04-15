#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // method can have same name as another value field
    // they will then be differentiated by the inclusion
    // of paretheses or not when used
    // often uses for creating getters
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // Associated function which creates a square
    // is not a method as it does not take self as a parameter
    // the Self keyword here is an alias for the type which appears after impl
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// it is valid syntax to have more than one impl block per type
impl Rectangle {
    fn another_rect(size: u32) -> Self {
        Self {
            width: size,
            height: size * 2,
        }
    }
}

fn main() {

    // Version 1 using simple variables
    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // Version 2 using a tuple, even less clear but groups the relevant data
    // let rect1 = (30, 50);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // );

    // fn area(dimensions: (u32, u32)) -> u32 {
    //     dimensions.0 * dimensions.1
    // }

    // Version 3 using a struct, much clearer

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {} square pixels.",
        // pass a reference here so main retains ownership of rect1
        rect1.area()
    );

    rect1.width();

    // calling an associated function must be done using the following syntax
    let square1 = Rectangle::square(5);

    // fn area(rectangle: &Rectangle) -> u32 {
    //     rectangle.height * rectangle.width
    // }

    // Adding useful functionality with derived traits
    // println! cannot be used below as the Rectangle struct
    // does not have the Display formatting like most primitives do by default

    // println!("rect1 is {}", rect1);

    // the compiler recommends the below, but this will not
    // work either as Rectangle also does not have the Debug trait by default
    // however, we can add it by deriving the trait we want above the struct itself ^^^

    println!("rect1 is {:?}", rect1);

    // the dbg! macro can also be used 
    dbg!(&rect1);
}
