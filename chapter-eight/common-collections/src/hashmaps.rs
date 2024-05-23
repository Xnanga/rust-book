use std::collections::HashMap;

pub fn run_hashmaps() {
  // Creating a new HashMap and inserting keys/values
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  // Retrieving a value
  let team_name = String::from("Blue");
  // get method returns an Option<&T> or None if no value found
  // copied() method turns Option<&i32> into an Option<i32>
  // unwrap_or() method turns score into 0 if no value found
  let _score = scores.get(&team_name).copied().unwrap_or(0);

  // Iterating over a hashmap
  for (key, value) in &scores {
    println!("{key}: {value}");
  }

  // Ownership
  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(field_name, field_value);
  // Hashmap now as ownership so trying to use either variable afterwards will cause an error
  // print!("{}", field_name);

  // Updating a HashMap
  // Overwriting an existing key/value
  // insert method will overwrite previous value if same key is written to
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25);

  println!("{:?}", scores);

  // Only updating key/value if it does not exist

  // entry method will check if key exists first before overwriting
  // or_insert method returns a mutable reference to the value for the entry if it exists
  // if value does not exist, it inserts the argument provided
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);

  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);

  println!("{:?}", scores);

  // Updating a value based on an old value
  // below takes each word, sets is as the key, gives it a value of 0 if it is not already in the table
  // it then increments the value for that key by one

  let text = "hello world wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      *count += 1;
  }

  println!("{:?}", map);
}