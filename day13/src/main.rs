use std::fs;

use aoc_utils::{AoCSolution, run};

struct Solution {}

fn print_page(points: &Vec<(u32,u32)>) {
  let (width, height) = points.iter().fold((0,0),|acc,p| (acc.0.max(p.0+1), acc.1.max(p.1+1)) );
  let mut array: Vec<bool> = vec![false; (width*height) as usize];

  for p in points {
    array[(p.0 + p.1 * width) as usize] = true;
  }

  for (i,b) in array.iter().enumerate() {
    if i % width as usize == 0 {
      println!();
    }
    if *b {
      print!("#");
    } else {
      print!(".");
    }
  }
  println!();
  println!();
}

impl AoCSolution for Solution  {
    fn part1(&self, input: &String) -> i64 {

      let mut points: Vec<(u32,u32)> = input.lines().take_while(|l| l.len() > 0).map(|l| {
        let splits: Vec<&str> = l.split(',').collect();
        return (splits[0].parse::<u32>().unwrap(),splits[1].parse::<u32>().unwrap());
      }).collect();

      let folds : Vec<(char,u32)> = input.lines().filter(|l| l.starts_with("fold") ).map(|l| {
        let splits: Vec<&str> = l.split('=').collect();
        return (splits[0].chars().last().unwrap(), splits[1].parse::<u32>().unwrap());
      }).collect();

      for (axis,fold_pos) in folds.into_iter().take(1) {
        match axis {
          'x' => {
            for p in &mut points {
              p.0 = if p.0 > fold_pos { 2*fold_pos - p.0 } else { p.0 };
            }
          },
          'y' => {
            for p in &mut points {
              p.1 = if p.1 > fold_pos { 2*fold_pos - p.1 } else { p.1 };
            }
          },
          _ => { panic!("unknown axis") }
        }
        points.sort();
        points.dedup();
      }

      return points.len() as i64;
    }
    
    fn part2(&self, input: &String) -> i64 {

      let mut points: Vec<(u32,u32)> = input.lines().take_while(|l| l.len() > 0).map(|l| {
        let splits: Vec<&str> = l.split(',').collect();
        return (splits[0].parse::<u32>().unwrap(),splits[1].parse::<u32>().unwrap());
      }).collect();

      let folds : Vec<(char,u32)> = input.lines().filter(|l| l.starts_with("fold") ).map(|l| {
        let splits: Vec<&str> = l.split('=').collect();
        return (splits[0].chars().last().unwrap(), splits[1].parse::<u32>().unwrap());
      }).collect();

      for (axis,fold_pos) in folds {
        match axis {
          'x' => {
            for p in &mut points {
              p.0 = if p.0 > fold_pos { 2*fold_pos - p.0 } else { p.0 };
            }
          },
          'y' => {
            for p in &mut points {
              p.1 = if p.1 > fold_pos { 2*fold_pos - p.1 } else { p.1 };
            }
          },
          _ => { panic!("unknown axis") }
        }
        points.sort();
        points.dedup();
      }

      print_page(&points);

      return 0 as i64;
    }
}

fn main() {
    run(
        "Advent of Code day 13", 
        &fs::read_to_string("input.txt").expect("Input Error"), 
        &Solution {}
    );
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;


  #[test]
  fn test_part1_1() {
    let input = String::from(indoc!{"
    6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0

    fold along y=7
    fold along x=5
    "});
    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 17);
  }

  #[test]
  fn test_part2_1() {
    let input = String::from(indoc!{"
    6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0

    fold along y=7
    fold along x=5
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 17);
  }

}