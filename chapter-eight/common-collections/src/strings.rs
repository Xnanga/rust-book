pub fn run_strings() {
  // Create a mutable string
  let mut _s = String::new();

  // This is a string literal (&srt)
  let data = "initial contents";

  // This turns that string literal into a String
  let _s = data.to_string();
  // This method also works on a literal directly:
    let _s = "initial contents".to_string();
  // We can also use from for this
  let _s = String::from("initial contents");

  // Appending to a string
  let mut s = String::from("foo");
  s.push_str("bar");

  // the push method adds just one character
  let mut s = String::from("lo");
  s.push('l');

  // Concatenation
  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

  // Can get unwieldy concatenating multiple strings
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  let _s = s1 + "-" + &s2 + "-" + &s3;

  // We can use the format! macro to simplify this
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  let _s = format!("{s1}-{s2}-{s3}");

  // Slicing strings
  // Generally not advised as return type will differ depending on the language/characters used
  // However, can be done by providing byte range to return
  let hello = "Здравствуйте";

  let _s = &hello[0..4];

  // The 3 above is a cyrillic character which takes up two bytes
  // If we tried to access the first byte of it, Rust would panic

  // Iterating over strings
  // Be explicit on what you are iterating over, chars or bytes

  // Chars
  for c in "Зд".chars() {
    println!("{c}");
  }

  // Bytes
  for b in "Зд".bytes() {
      println!("{b}");
  }
}