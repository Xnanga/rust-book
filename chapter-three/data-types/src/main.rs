fn main() {
    // Scalar types
        // integers (signed/unsigned, 8-bit to 128-bit and arch)
        let x = 2
        let y: u32 = 5
        let z: i32 = -6

        // floating point
        let a = 2.0 // f64 (default)
        let b: f32 = 3.0 //f32

        // boolean
        let t = true
        let: bool = false

        // char
        let c = 'z'
        let d: char = 'Z'

    // Compound types
        // tuples (define all types in order they appear)
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup;
        println!("The value of y is: {y}");

        // arrays (all same type, immutable length)
        let arr = [1, 2, 3, 4, 5];
        let annotated_arr: [i32; 3] = [6, 7, 8];
        let first = arr[0]
        // accessing missing element will crash a Rust program due to memory safety features
        // let missing = arr[10]
}
