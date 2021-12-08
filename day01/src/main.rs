use std::fs;

use aoc_utils::{AoCSolution, run};

struct Solution {}

impl AoCSolution for Solution  {
    fn part1(&self, input: &String) -> i64 {
        let instructions = input.lines().map(|l| {
            return l.parse::<u64>().unwrap();
        }).collect::<Vec<u64>>();
    
        let sum = instructions.into_iter().fold((u64::MAX,0), |(prev, acc), val| {
            return if prev < val {(val, acc+1)} else {(val, acc)};
        });
    
        return sum.1
    }
    
    fn part2(&self, input: &String) -> i64 {
        let instructions = input.lines().map(|l| {
            return l.parse::<u64>().unwrap();
        }).collect::<Vec<u64>>();
    
        let added_measurements = instructions.into_iter().enumerate().fold(vec![], |mut acc, (i, val)| {
            if i>1 {
                acc[i-2] += val;
            }
            if i>0 {
                acc[i-1] += val;
            }
            acc.push(val);
            return acc;
        });
    
        let sum = added_measurements[..added_measurements.len()-2].into_iter().fold((u64::MAX,0), |(prev, acc), &val| {
            return if prev < val {(val, acc+1)} else {(val, acc)};
        });
    
        return sum.1;
    }
}

fn main() {
    run(
        "Advent of Code day 01", 
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
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    "});

    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 7);
  }

  #[test]
  fn test_part_2() {
    let input = String::from(indoc!{"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 5);
  }
}