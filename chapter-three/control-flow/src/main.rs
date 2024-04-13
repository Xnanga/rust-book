fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    secondary();
}

fn secondary() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // result of if/else statements must be of same type
    // let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}