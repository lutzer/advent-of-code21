use std::fs;
use std::collections::HashMap;

use aoc_utils::{AoCSolution, run};

struct Solution {}

#[derive(Debug)]
struct Instruction {
    from: (u32,u32),
    to: (u32,u32)
}

impl AoCSolution for Solution  {
    fn part1(&self, input: &String) -> i64 {
        let instructions = input.lines().map(|l| {
            let s = l.replace(&['>', ' '][..], "");
            let splits: Vec<&str> = s.split(&['-',','][..]).collect();
            return Instruction {
                from: (splits[0].parse::<u32>().unwrap(), splits[1].parse::<u32>().unwrap()),
                to: (splits[2].parse::<u32>().unwrap(), splits[3].parse::<u32>().unwrap())
            }
        }).collect::<Vec<Instruction>>();

        let mut map : HashMap<(u32,u32), u32> = HashMap::new();
        for instruction in instructions {
            let x1 = instruction.from.0.min(instruction.to.0);
            let x2 = instruction.from.0.max(instruction.to.0);
            let y1 = instruction.from.1.min(instruction.to.1);
            let y2 = instruction.from.1.max(instruction.to.1);
            if y1==y2 {
                for x in x1..x2+1 {
                    *map.entry((x,y1)).or_insert(0) += 1
                }
            } else if x1==x2 {
                for y in y1..y2+1 {
                    *map.entry((x1,y)).or_insert(0) += 1
                }
            }
        }

        let overlap_count = map.iter().fold(0,|acc,curr| {
            return if *curr.1 > 1 { acc+1 } else { acc }
        });

        return overlap_count;
    }
    
    fn part2(&self, input: &String) -> i64 {
        let instructions = input.lines().map(|l| {
            let s = l.replace(&['>', ' '][..], "");
            let splits: Vec<&str> = s.split(&['-',','][..]).collect();
            return Instruction {
                from: (splits[0].parse::<u32>().unwrap(), splits[1].parse::<u32>().unwrap()),
                to: (splits[2].parse::<u32>().unwrap(), splits[3].parse::<u32>().unwrap())
            }
        }).collect::<Vec<Instruction>>();

        let mut map : HashMap<(u32,u32), u32> = HashMap::new();
        for instruction in instructions {
            let x1 = instruction.from.0.min(instruction.to.0);
            let x2 = instruction.from.0.max(instruction.to.0);
            let y1 = instruction.from.1.min(instruction.to.1);
            let y2 = instruction.from.1.max(instruction.to.1);
            if y1==y2 {
                for x in x1..x2+1 {
                    *map.entry((x,y1)).or_insert(0) += 1
                }
            } else if x1==x2 {
                for y in y1..y2+1 {
                    *map.entry((x1,y)).or_insert(0) += 1
                }
            } else {
                let x_up = instruction.to.0 as i32 - instruction.from.0 as i32 > 0;
                let y_up = instruction.to.1 as i32 - instruction.from.1 as i32 > 0;
                for i in 0..(x2-x1)+1 {
                    let x = if x_up {instruction.from.0 + i} else {instruction.from.0 - i};
                    let y = if y_up {instruction.from.1 + i} else {instruction.from.1 - i};
                    *map.entry((x,y)).or_insert(0) += 1
                }
            }
        }

        let overlap_count = map.iter().fold(0,|acc,curr| {
            return if *curr.1 > 1 { acc+1 } else { acc }
        });

        return overlap_count;
    }
}

fn main() {
    run(
        "Advent of Code day 05", 
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
    0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2
    "});

    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 5);
  }

  #[test]
  fn test_part_2() {
    let input = String::from(indoc!{"
    0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 12);
  }
}