pub fn run_vectors() {
  // Creating a vector
  let _w: Vec<i32> = Vec::new();

  // Macro which creates a vector with items already in it
  // Type is inferred
  let mut v = vec![1, 2, 3];

  // We would not be able to push to the vector if code below were uncommented
  // this is because we cannot have both mutable and unmutable references in the same scope for a variable
  // let first = &v[0];

  // Push method
  v.push(4);
  v.push(5);
  v.push(6);

  // Reading elements from vectors
  // Method one
  let third: &i32 = &v[2];
  println!("The third element is {third}");

  // Method two
  let fourth: Option<&i32> = v.get(3);
  match fourth {
    Some(fourth) => println!("The fourth element is {fourth}"),
    None => println!("There is no fourth element."),
  };

  // Accessing a non-existent element

  // Rust will panic here
  // let _does_not_exist_one = &v[100];

  // This will return None
  let _does_not_exist_two = v.get(100);

  // Iterating over a vector

  // With a for loop
  // immutable
  let x = vec![100, 32, 57];
    for i in &x {
        println!("{i}");
    }

  // mutable
  let mut y = vec![100, 32, 57];
    for i in &mut y {
      // * dereferences to pull out the value from the reference
        *i += 50;
    }

    // Storing different types in a vector using enums
    enum SpreadsheetCell {
      Int(i32),
      Float(f64),
      Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
