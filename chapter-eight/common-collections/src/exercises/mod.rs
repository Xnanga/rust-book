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

struct Employee {
  name: String,
  department: String,
}

fn init_employee_db() {
  let mut employee_db: HashMap<String, Vec<String>> = HashMap::new();

  // Happy Paths
  add_employee(&mut employee_db, "Add Nelly to Marketing");
  add_employee(&mut employee_db, "Add Samantha to Engineering");
  add_employee(&mut employee_db, "Add James to Marketing");

  // Sad Paths
  add_employee(&mut employee_db, "Add James to");
  add_employee(&mut employee_db, "One Two Three Four");
  add_employee(&mut employee_db, "");

  get_all_employees_by_department(&mut employee_db, "Marketing");

  println!("{:?}", employee_db);
}

fn add_employee(db: &mut HashMap<String, Vec<String>>, command: &str) -> () {
  let lowercase_command = command.to_lowercase();
  let mut command_words = lowercase_command.split_whitespace();
  let add_command = command_words.next();
  let name = command_words.next();
  let to_command = command_words.next();
  let department = command_words.next();

  if !add_command.is_some_and(|cmd| cmd == "add") || !to_command.is_some_and(|cmd| cmd == "to") { return }

  match(name, department) {
    (Some(name), Some(department)) => {
      let new_employee = Employee { name: name.to_string(), department: department.to_string() };
      
      if db.contains_key(&new_employee.department) {
        let entry = db.entry(new_employee.department);
        entry.or_insert_with(Vec::new).push(new_employee.name);
      } else {
        db.insert(new_employee.department, vec![new_employee.name]);
      }
    },
    _ => return
  }
}

fn get_all_employees_by_department(db: &mut HashMap<String, Vec<String>>, department: &str) -> Vec<String> {
  // TODO: Figure out how to get Vec from HashMap or default to empty Vec if not found
  // let entry = db.get(department).unwrap_or_else(|| Vec::new)
  vec!["dummyName".to_string()]
}

pub fn run_all_exercises() {
  // Exercise One
  let random_numbers = vec![6, 16, 20, 21, 24, 24, 24, 45, 60, 66, 77, 83, 83, 94, 99];
  let median_and_mode = retrieve_median_and_mode(&random_numbers);
  println!("{:?}", median_and_mode);

  // Exercise Two
  let random_sentence = "The quick brown fox jumped over the lazy dog";
  let pig_latin_sentence = convert_to_pig_latin(&random_sentence);
  println!("{}", pig_latin_sentence);

  // Exercise Three
  init_employee_db();
}