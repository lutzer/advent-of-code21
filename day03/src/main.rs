use std::fs;

use aoc_utils::{AoCSolution, run};

struct Solution {}

fn bitstring_to_u32(s: &str) -> u32 {
    let mut result = 0;
    for (i,c) in s.chars().rev().enumerate() {
        if c == '1' {
            result += (2 as u32).pow(i as u32)
        }
    }
    return result;
}

impl AoCSolution for Solution  {
    fn part1(&self, input: &String) -> i64 {
        let instructions = input.lines().map(|l| {
            return l
        }).collect::<Vec<&str>>();

        let number_size = instructions[0].len();
        let size = instructions.len() as u32;

        let bit_counts : Vec<u32> = instructions.into_iter().fold(vec![0;number_size], |mut acc, curr| {
            for (i,c) in curr.chars().enumerate() {
                if c == '1' { acc[i] += 1}
            }
            return acc
        });

        let (gamma_rate, epsilon_rate) = bit_counts.into_iter().fold(("".to_string(), "".to_string()),|mut acc,c| {
            acc.0 = if c > size/2 { acc.0 + "1" } else { acc.0 + "0" };
            acc.1 = if c <= size/2 { acc.1 + "1" } else { acc.1 + "0" };
            return acc;
        });

        return (bitstring_to_u32(&gamma_rate.to_owned()) * bitstring_to_u32(&epsilon_rate.to_owned())) as i64;
    }
    
    fn part2(&self, input: &String) -> i64 {

        let instructions = input.lines().map(|l| {
            return l
        }).collect::<Vec<&str>>();

        let number_size = instructions[0].len();

        let mut oxygen_ratings = instructions.to_vec();
        for i in 0..number_size {
            let bit_count = (&oxygen_ratings).into_iter().fold(0, |acc, curr| {
                if curr.chars().nth(i).unwrap() == '1' { acc + 1 } else { acc } 
            });
            let most_common = if bit_count as f32 >= oxygen_ratings.len() as f32/2.0 { '1' } else { '0' };
            oxygen_ratings.retain(|&x| {
                return x.chars().nth(i).unwrap() == most_common
            });
            if oxygen_ratings.len() == 1 {
                break;
            }
        }

        let mut co2_ratings = instructions.to_vec();
        for i in 0..number_size {
            let bit_count = (&co2_ratings).into_iter().fold(0, |acc, curr| {
                if curr.chars().nth(i).unwrap() == '1' { acc + 1 } else { acc } 
            });
            let least_common = if (bit_count as f32) < co2_ratings.len() as f32/2.0 { '1' } else { '0' };
            co2_ratings.retain(|&x| {
                return x.chars().nth(i).unwrap() == least_common
            });
            if co2_ratings.len() == 1 {
                break;
            }
        }

        return (bitstring_to_u32(&oxygen_ratings[0].to_owned()) * bitstring_to_u32(&co2_ratings[0].to_owned())) as i64;
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
    00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010
    "});

    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 198);
  }

  #[test]
  fn test_part_2() {
    let input = String::from(indoc!{"
    00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 230);
  }
}