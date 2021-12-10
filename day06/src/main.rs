use std::fs;

use aoc_utils::{AoCSolution, run};

struct Solution {}

#[derive(Debug)]
struct Instruction {
    from: (u32,u32),
    to: (u32,u32)
}

impl AoCSolution for Solution  {
    fn part1(&self, input: &String) -> i64 {
        let mut lanternfishes : Vec<u32> = input.trim_end().split(",").map(|x| {
            return x.parse::<u32>().unwrap();
        }).collect();

        for _ in 0..80 {
            let mut newborns = 0;
            for f in &mut lanternfishes {
                if *f == 0 {
                    newborns += 1;
                    *f = 6;
                }  else {
                    *f -= 1;
                }
            }
            for _ in 0..newborns {
                lanternfishes.push(8);
            }
        }

        return lanternfishes.len() as i64;
    }
    
    fn part2(&self, input: &String) -> i64 {
        let lanternfishes : Vec<u32> = input.trim_end().split(",").map(|x| {
            return x.parse::<u32>().unwrap();
        }).collect();

        let mut age_bins: Vec<u64> = vec![0;9];

        for f in lanternfishes {
            age_bins[f as usize] += 1;
        }

        for d in 0..256 {
            let mothers = age_bins.remove(0);
            age_bins[6] += mothers;
            age_bins.push(mothers); // newborns
        }

        let count = age_bins.iter().fold(0,|acc,curr| acc + curr );

        return count as i64;
    }
}

fn main() {
    run(
        "Advent of Code day 06", 
        &fs::read_to_string("input.txt").expect("Input Error"), 
        &Solution {}
    );
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  #[test]
  fn test_parts() {
    let input = String::from(indoc!{"
    3,4,3,1,2
    "});

    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 5934);
  }

  #[test]
  fn test_part_2() {
    let input = String::from(indoc!{"
    3,4,3,1,2
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 26984457539);
  }
}