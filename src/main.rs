use std::{cmp::max, fs::read_to_string};

fn get_gameindex_from_line(line: &str) -> u32
{
  let gametext = line.strip_prefix("Game ").unwrap().split_once(":");
  gametext.unwrap().0.parse().unwrap()
}

fn get_rgb(line: &str) -> (u32,u32,u32)
{
  let mut red = 0;
  let mut green = 0;
  let mut blue = 0;
  let rolls= line.split_once(":").unwrap().1.split(";");
  for roll in rolls {
    let onecolor = roll.split(",");
    for num_plus_color in onecolor {
      let split = num_plus_color.strip_prefix(" ").unwrap().split_once(" ");
      let curr_num = split.unwrap().0.parse().unwrap();
      let curr_color = split.unwrap().1;
      match curr_color {
        "green"=> green = max(green, curr_num),
        "red"=> red = max(red, curr_num),
        "blue"=> blue = max(blue, curr_num),
        &_ => todo!()          
      }
    }
  }
  (red,green,blue)
}

fn is_game_possible (rgb:(u32,u32,u32)) -> bool{
  let max_red = 12;
  let max_green = 13;
  let max_blue = 14;
  rgb.0 <= max_red && rgb.1 <= max_green && rgb.2 <= max_blue
}

fn get_sum_from_all_lines(filename: &str) -> u32 {
  let mut result  = 0;

  for line in read_to_string(filename).unwrap().lines() {
    let gameidx = get_gameindex_from_line(line);
    let rgb = get_rgb(line);
    if is_game_possible(rgb){
      result += gameidx;
    }
  }
  return result
}

fn main() {
  let _a = get_sum_from_all_lines("games.txt");  
  println!("total result {}", _a);
}
