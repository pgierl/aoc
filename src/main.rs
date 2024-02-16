use std::fs::read_to_string;

//get first integer from string
// a1b2c3d4e5f returns 1+5 -> 15
fn get_sum_first_last_int(line: &str) -> u32
{
  let first_int = line.chars().find(char::is_ascii_digit);
  let first_char: char =    match first_int {
    Some(value) =>  value,
    None => ' '
  };
  let last_int = line.chars().rfind(char::is_ascii_digit);
  let last_char: char =    match last_int {
    Some(value) =>  value,
    None => ' '
  };
  let mut string_both_chars = String::from("");
  string_both_chars.push(first_char);
  string_both_chars.push(last_char);
  return string_both_chars.parse().unwrap()
}

//get first integer from string
// eightwothree returns -> 83
// xtwone3four ->24
fn get_sum_first_last_int_vals_as_word(line: &str) -> u32
{
  let mut tmp = line.replace("one", "one1one");
  tmp = tmp.replace("two", "two2two");
  tmp = tmp.replace("three", "three3three");
  tmp = tmp.replace("four", "four4four");
  tmp = tmp.replace("five", "five5five");
  tmp = tmp.replace("six", "six6six");
  tmp = tmp.replace("seven", "seven7seven");
  tmp = tmp.replace("eight", "eight8eight");
  tmp = tmp.replace("nine", "nine9nine");
  return get_sum_first_last_int(&tmp);
}


fn get_sum_from_all_lines(filename: &str) -> u32 {
  let mut result  = 0;
  for line in read_to_string(filename).unwrap().lines() {
    println!("{} = {}", line.to_string(), get_sum_first_last_int_vals_as_word(&line));
    result += get_sum_first_last_int_vals_as_word(&line)
  }
  return result
}

fn main() {
  let _a = get_sum_from_all_lines("1.txt");  
  println!("total result {}", _a);
}
