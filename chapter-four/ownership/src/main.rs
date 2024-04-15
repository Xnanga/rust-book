// Ownership rules
// 1. Every value has an owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value will be dropped

fn main() {
    println!("Hello, world!");

    { // s is not valid as it is out of scope
        let _s = "hello"; // s is valid from this point as it has been declared
        // do stuff with s
    } // this scope is now over, s is no longer valid as it has been removed from memory

    // string literal, stored on the stack
    // let a = "hello";

    // string type, stored on the heap
    let mut a = String::from("hello");

    a.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", a); // This will print `hello, world!`

    // Memory and Allocation
    // below example creates a copy of a scala value (like pass-by-value)
    let _y = 5; // contains 5
    let _z = a; // also contains 5

    // below example both point to the same memory allocation in the heap
    let my_first_string = String::from("Hello"); // points to data found at 0x7FFF5FBFFD98
    let _my_second_string = my_first_string; // also points to data found at 0x7FFF5FBFFD98

    // at this point, my_first_string is no longer valid as it has been dropped
    // from memory as my_second_string now takes its place
    // this prevents a double-free error where 0x7FFF5FBFFD98 could be cleared twice

    // if you do want a 'deep copy' of a variable, this can be done with clone
    let s1 = String::from("Greetings");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // as ownership of variables is taken by functions which receive them as variables
    // the variable will need to be returned if the first function still needs that variable
    // after having passed it to function two, tuples are one option for doing this
    fn primary_fn() {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length_one(s1);

        println!("The length of '{}' is {}.", s2, len);
    }

    fn calculate_length_one(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
    
        (s, length)
    }

    primary_fn();

    // however, a better way to do this is usually by providing a reference to the value instead
    // this lets a function "borrow" the value without then having to return it manually
    // note the ampersands used denoting a reference, referring to a value without transferring ownership of it

    fn secondary_fn() {
        let s1 = String::from("hello");

        let len = calculate_length_two(&s1);

        println!("The length of '{}' is {}.", s1, len);

        // the below function would not work due to not transferring ownership
        // change(&s1);
        // however, we can fix this by passing a mutable reference e.g.
        let mut mutable_string = String::from("Hello");
        mut_change(&mut mutable_string);
        println!("{}", mutable_string);
        // note: you can only have one mutable reference to a value
    }

    fn calculate_length_two(s: &String) -> usize { // &String denotes that the function expects a reference to a String
        s.len()
    } // s goes out of scope here but is not dropped as this function does not have ownership of s

    // this function would not work as functions cannot mutate variables that they do not have ownership of
    // fn change(some_string: &String) {
    //     some_string.push_str(", world");
    // }

    // this function would work as a mutable reference has been passed to it
    fn mut_change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    secondary_fn();

    // Dangling Pointers
    // Rust won't allow the referencing of values which are no longer available in memory
    // the example below would create a dangling pointer, which the compiler will catch

    fn create_reference() {
        let _reference_to_nothing = dangle();
    }

    // fn dangle() -> &String {
    //     let s = String::from("Hello");

    //     &s
    // }

    // the fix here would be to return the string itself, not a reference to it
    fn dangle() -> String {
        let s = String::from("Hello");

        s
    }

    create_reference();
}
