fn main() {
    
    // V1 - returns unsigned int
    // fn first_word(s: &String) -> usize {
    //     let bytes = s.as_bytes();
    
    //     for (i, &item) in bytes.iter().enumerate() {
    //         if item == b' ' {
    //             return i;
    //         }
    //     }
    
    //     s.len()
    // }

    // V1 - returns string slice reference
    // also can take both strings and string slices as argument now due to deref coercion
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
    
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
    
        &s[..]
    }

    let s = String::from("hello world");

    let _word = first_word(&s); // word will get the value 5

    // this empties the String, making it equal to ""
    // s.clear();

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // String slices solve this by referencing part of the string instead
    let _hello = &s[0..5];
    let _world = &s[6..11];

    // placing no starting index or no ending index will slice everything
    // from the beginning and end of the string respectively
    let _hello_two = &s[..5];
    let _world_two = &s[6..];
    // placing no indices will slice the entire string
    let _full_str = &s[..];

    // Array slices are also possible in much the same way
    let my_arr = [1, 2, 3, 4, 5, 6];
    let _my_arr_slice = &my_arr[1..3];
    assert_eq!(_my_arr_slice, &[2, 3]);
}
