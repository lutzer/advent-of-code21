use std::fs;

use aoc_utils::{AoCSolution, run};

struct Solution {}

impl AoCSolution for Solution  {
    fn part1(&self, input: &String) -> i64 {
        let crab_positions : Vec<u32> = input.trim_end().split(",").map(|x| {
            return x.parse::<u32>().unwrap();
        }).collect();

        fn calculate_fuel_consumption(target: u32, positions: &Vec<u32>) -> u32 {
            let mut fuel = 0;
            for p in positions {
                fuel += if *p > target { p-target } else { target-p };
            }
            return fuel
        }
        
        fn median(input: &Vec<u32>) -> u32 {
            let mut mut_input = input.clone();
            mut_input.sort();
        
            let mid = mut_input.len() / 2;
            if mut_input.len() % 2 == 0 {
                (mut_input[mid - 1] + mut_input[mid])/2 as u32
            } else {
                mut_input[mid]
            }
        }

        let fuel_consumption = calculate_fuel_consumption(median(&crab_positions), &crab_positions);

        return fuel_consumption as i64;
    }
    
    fn part2(&self, input: &String) -> i64 {
        let crab_positions : Vec<u32> = input.trim_end().split(",").map(|x| {
            return x.parse::<u32>().unwrap();
        }).collect();

        fn calculate_fuel_consumption(target: u32, positions: &Vec<u32>) -> u32 {
            let mut fuel = 0;
            for p in positions {
                let dist = if *p > target { p-target } else { target-p };
                fuel += (dist * (dist+1)) / 2
            }
            return fuel
        }

        let (min, max) = crab_positions.iter().fold((u32::MAX,0), |(min,max),&curr| {
            return (curr.min(min), curr.max(max));
        });

        let fuel_consumptions = (min..max).map(|t| {
            return (t,calculate_fuel_consumption(t, &crab_positions));
        });

        let min_consumption = fuel_consumptions.fold(u32::MAX, |acc, f| {
            return acc.min(f.1);
        });

        return min_consumption as i64;
    }
}

fn main() {
    run(
        "Advent of Code day 07", 
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
    16,1,2,0,4,2,7,1,2,14
    "});

    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 37);
  }

  #[test]
  fn test_part_2() {
    let input = String::from(indoc!{"
    16,1,2,0,4,2,7,1,2,14
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 168);
  }
}