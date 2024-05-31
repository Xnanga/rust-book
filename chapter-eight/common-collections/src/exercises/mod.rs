/*
  1. Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) 
  and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/

use std::collections::HashMap;

fn get_median_value(num_list: &[u32]) -> u32 {
  let num_list_len = num_list.len();
  if num_list_len == 1 { return 0 };

  let mut sorted_nums = num_list.to_vec();
  sorted_nums.sort();
  let middle_value = num_list_len / 2;
  let is_even_number_of_values: bool = num_list_len % 2 == 0;

  match is_even_number_of_values {
    false => sorted_nums[middle_value],
    true => (sorted_nums[middle_value - 1] + sorted_nums[middle_value]) / 2,
  }
}

fn get_mode_value(num_list: &[u32]) -> u32 {
  let mut value_frequencies = HashMap::new();

  for num in num_list {
    let count = value_frequencies.entry(num).or_insert(0);
    *count += 1;
  }

  let mut mode: u32 = 0;
  for (key, value) in value_frequencies {
    if value > mode { mode = *key }
  }

  mode
}

fn retrieve_median_and_mode(num_list: &[u32]) -> HashMap<String, u32> {
  let mut median_and_mode = HashMap::new();
  median_and_mode.insert(String::from("median"), get_median_value(num_list));
  median_and_mode.insert(String::from("mode"), get_mode_value(num_list));
  median_and_mode
}

/*
  2. Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, 
  so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
  Keep in mind the details about UTF-8 encoding!
*/

fn convert_to_pig_latin(text: &str) -> String {
  let vowels = vec!['a', 'e', 'i', 'o', 'u'];
  let lowercase_text = text.to_lowercase();
  let list_of_words = lowercase_text.split_whitespace();
  let mut pig_latin_list: Vec<String> = Vec::new();

  for word in list_of_words {
    let first_char = word.chars().nth(0);
    match first_char {
      Some(char) => {
        if vowels.contains(&char) { pig_latin_list.push(word.to_owned() + "-hay") } 
        else {
          let mut new_word = word.to_owned() + "-" + &char.to_string() + "ay";
          // This will panic if a character requiring more than one byte is included
          // find a way to iterate over the chars and remove the first one, then back to string
          new_word.remove(0);
          pig_latin_list.push(new_word)
        }
      }
      None => continue,
    }
  }

  pig_latin_list.join(" ")
}

/*
  3. Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. 
  For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department 
  or all people in the company by department, sorted alphabetically.
*/


pub fn run_all_exercises() {
  // Exercise One
  let random_numbers = vec![6, 16, 20, 21, 24, 24, 24, 45, 60, 66, 77, 83, 83, 94, 99];
  let median_and_mode = retrieve_median_and_mode(&random_numbers);
  println!("{:?}", median_and_mode);

  // Exercise Two
  let random_sentence = "The quick brown fox jumped over the lazy dog";
  let pig_latin_sentence = convert_to_pig_latin(&random_sentence);
  println!("{}", pig_latin_sentence);
}