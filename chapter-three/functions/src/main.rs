fn main() {
    print_labeled_measurement(5, 'h');

    let x = five();

    println!("The value of x is: {x}");
}

// parameter types must be specified in each function
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// arrow used to denote return type
fn five() -> i32 {
    // return keyword can be used to return value early
    // return 5

    // last value in function is returned, like Scala
    // expressions do not require semicolon, otherwise they will 
    // be considered statements which do not return any value
    5
}