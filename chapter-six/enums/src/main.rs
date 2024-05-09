#![allow(unused)]
fn main() {
    // IpAddrKind is a type with two variants
    enum IpAddrKind {
        V4,
        V6,
    }

    // IpAddr is a type with two variants that accept data
    enum IpAddr {
        // Adding a type value allows us to create an instance of this type in much the same way we would use a struct
        V4(u8, u8, u8, u8), // An advantage over structs is being able to pass different data to each type variant
        V6(String),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // Defining methods on enums is the same as on structs
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    // Alternative way of implementing Message variants with structs which is more verbose
    // and makes it more difficult to create functions which support this type
    // struct QuitMessage; // unit struct
    // struct MoveMessage {
    //     x: i32,
    //     y: i32,
    // }
    // struct WriteMessage(String); // tuple struct
    // struct ChangeColorMessage(i32, i32, i32); // tuple struct

    // Creating instances of an enum type
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Defining a function which can accept any variant of IpAddrKind
    fn route(ip_kind: IpAddrKind) {}
    route(four);
    route(six);


    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // The Option enum - looks like the below (similar to Scala)
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    // Use of Some and None with different types
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // The match Control Flow Construct (pattern matching)

    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
    }

    // Different types of Coin
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    
    // Function which can take any kind of coin and return its value as a integer
    // Matches against type of Coin and returns the correct value
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            // "state" variable binds to the value of this quarter's state
            // allowing it to be accessed and used in this path
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    let my_quarter = Coin::Quarter(UsState::Alabama);

    value_in_cents(my_quarter);

    // Using Option<T> in pattern matching

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            // We must handle all possible patterns or this can lead to bugs
            // The compiler will often detect when patterns are not exhaustive
            // If we remove the line which handles None, the compiler would complain
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Catch-all patterns and the _ placeholder

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // Below is catch-all value
        // other => move_player(other),
        // Below is catch-all scenario when we don't want to use the value
        // _ => reroll(),
        // Below is a catch-all when we don't want to do anything
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    // fn move_player(num_spaces: u8) {}
    // fn reroll() {}

    // The if left Pattern

    // Code below works and covers all bases, but comes with additional boilerplate
    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }

    let config_max = Some(3u8);
    // Syntactic sugar for checking if one value exists and running code if it does
    // You lose the exhaustive pattern checks that match provides, however
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // Using else with an if let

    // Else can be used to effectively handle every value other than the one pattern match defined in the if let

    // This match can be converted to an if let else
    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    // 
    // let mut count = 0;
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     count += 1;
    // }
}
