use std::fs;

use aoc_utils::{AoCSolution, run};

struct Solution {}

impl AoCSolution for Solution  {
    fn part1(&self, input: &String) -> i64 {
        let instructions = input.lines().map(|l| {
            let words = l.split(" ").collect::<Vec<&str>>();
            return (words[0], words[1].parse::<u64>().unwrap());
        }).collect::<Vec<(&str,u64)>>();

        let (hpos,depth) = instructions.into_iter().fold((0,0), |(h,d),val| {
            match val.0 {
                "forward" => (h+val.1, d),
                "down" => (h, d+val.1),
                "up" => (h, d-val.1),
                _ => (h,d)
            }
        });

        return (hpos*depth) as i64;
    }
    
    fn part2(&self, input: &String) -> i64 {
        let instructions = input.lines().map(|l| {
            let words = l.split(" ").collect::<Vec<&str>>();
            return (words[0], words[1].parse::<u64>().unwrap());
        }).collect::<Vec<(&str,u64)>>();

        let (hpos,depth,aim) = instructions.into_iter().fold((0,0,0), |(h,d,a),val| {
            match val.0 {
                "forward" => (h+val.1, d+a*val.1, a),
                "down" => (h, d, a+val.1),
                "up" => (h, d, a-val.1),
                _ => (h, d, a)
            }
        });


        return (hpos*depth) as i64;
    }
}

fn main() {
    run(
        "Advent of Code day 02", 
        &fs::read_to_string("input.txt").expect("Input Error"), 
        &Solution {}
    );
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  #[test]
  fn test_part_1() {
    let input = String::from(indoc!{"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "});

    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 150);
  }

  #[test]
  fn test_part_2() {
    let input = String::from(indoc!{"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 900);
  }
}