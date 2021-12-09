use std::fs;

use aoc_utils::{AoCSolution, run};

struct Solution {}

type Board = Vec<(u32, bool)>;

fn has_won(board: &Board) -> bool {
    for i in 0..5 {
        // check rows
        let has_false_rows = (0..5).any(|j| {
            return !board[i*5+j].1
        });
        if !has_false_rows {
            return true
        }
        // check cols
        let has_false_cols = (0..5).any(|j| {
            return !board[j*5+i].1
        });
        if !has_false_cols {
            return true
        }
    }
    return false
}

fn calculate_sum(board: &Board) -> u32 {
    return board.iter().fold(0,|acc,curr| {
        return if !curr.1 { acc + curr.0 } else { acc }
    })
}

impl AoCSolution for Solution  {
    fn part1(&self, input: &String) -> i64 {
        let instructions = input.lines().map(|l| {
            return l
        }).collect::<Vec<&str>>();

        let drawn_numbers: Vec<u32> = instructions[0].split(",").map(|n| {
            return n.parse::<u32>().unwrap();
        }).collect();

        let mut boards = instructions.into_iter().skip(1).fold(vec![],|mut boards: Vec<Board>, curr| {
            if curr == "" {
                boards.push(vec![]);
            } else {
                let length = boards.len();
                for n in curr.split(" ") {
                    let number = n.parse::<u32>();
                    if number.is_ok() {
                        boards[length-1].push((number.unwrap(), false));
                    }
                }
            }
            return boards;
        });

        for &dn in &drawn_numbers {
            for b in &mut boards {
                for n in b {
                    n.1 = (n.0 == dn) || n.1;
                }
            }
            for b in &boards {
                if has_won(b) {
                    return (calculate_sum(&b) * dn) as i64
                }
            }
        }

        return 0;
    }
    
    fn part2(&self, input: &String) -> i64 {
        let instructions = input.lines().map(|l| {
            return l
        }).collect::<Vec<&str>>();

        let drawn_numbers: Vec<u32> = instructions[0].split(",").map(|n| {
            return n.parse::<u32>().unwrap();
        }).collect();

        let mut boards = instructions.into_iter().skip(1).fold(vec![],|mut boards: Vec<Board>, curr| {
            if curr == "" {
                boards.push(vec![]);
            } else {
                let length = boards.len();
                for n in curr.split(" ") {
                    let number = n.parse::<u32>();
                    if number.is_ok() {
                        boards[length-1].push((number.unwrap(), false));
                    }
                }
            }
            return boards;
        });

        for &dn in &drawn_numbers {
            for b in &mut boards {
                for n in b {
                    n.1 = (n.0 == dn) || n.1;
                }
            }
            if boards.len() > 1 {
                boards.retain(|b| {
                    return !has_won(b)
                });
            } else if has_won(&boards[0]) {
                return (calculate_sum(&boards[0]) * dn) as i64
            }
        }

        return 0;
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
    7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
    8  2 23  4 24
    21  9 14 16  7
    6 10  3 18  5
    1 12 20 15 19

    3 15  0  2 22
    9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
    2  0 12  3  7
    "});

    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 4512);
  }

  #[test]
  fn test_part_2() {
    let input = String::from(indoc!{"
    7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
    8  2 23  4 24
    21  9 14 16  7
    6 10  3 18  5
    1 12 20 15 19

    3 15  0  2 22
    9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
    2  0 12  3  7
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 1924);
  }
}