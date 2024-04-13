fn main() {
    // constants cannot be made mutable and must have its type annotation
    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // using 'let' also makes variables immutable but rust can infer the type automatically
    let _w = 2;

    // the 'mut' keyword must be used if the value is to be mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


    // shadowing a variable means we create a new variable with the same name as a previous one
    // the new variable is used either until it itself is shadowed or the code scope ends

    // 1. Initial variable
    let y = 5;

    // 2. variable is shadowed
    let y = y + 1;

    {
        // 3. variable is shadowed again until end of this scope
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    // by this point, the variable has returned to what it was at point 2
    println!("The value of y is: {y}");
}