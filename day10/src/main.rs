use std::fs;

use aoc_utils::{AoCSolution, run};

struct Solution {}

fn get_closing_character(c: &char) -> Option<char> {
    let pairs = vec![('(',')'), ('[',']'), ('{','}'), ('<','>')];
    return pairs.iter().find(|(open,close)| *open == *c).map(|p| p.1);
}

impl AoCSolution for Solution  {
    fn part1(&self, input: &String) -> i64 {

        // returns (index, found, expected)
        fn find_first_illegal_character_index(s: String) -> Option<(usize,char,char)> {
            let mut opened : Vec<char> = vec![];
            
            for (i,c) in s.chars().enumerate() {
                match c {
                    '(' | '[' | '{' | '<' => { opened.push(c); },
                    ')' | ']' | '}' | '>' => {
                        if let Some(last) = opened.pop() {
                            let expected = get_closing_character(&last).unwrap();
                            if c != expected {
                                return Some((i, c, expected));
                            }
                        } else {
                            return Some((i, ' ', ' '));
                        }
                    },
                    _ => { }
                }
            }
            return None;
        }

        let lines = input.lines();

        let illegal_chars = lines.map(|l| find_first_illegal_character_index(l.to_string()) )
            .collect::<Vec<Option<(usize,char,char)>>>();

        let score = illegal_chars.iter().fold(0, |acc, error| {
            if let Some((_, found, _)) = error {
                return acc + match found {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => 0
                }
            } else {
                return acc;
            }
        });

        return score as i64;
    }
    
    fn part2(&self, input: &String) -> i64 {

        fn find_incomplete_lines(s: String) -> Option<Vec<char>> {
            let mut opened : Vec<char> = vec![];
            
            for (i,c) in s.chars().enumerate() {
                match c {
                    '(' | '[' | '{' | '<' => { opened.push(c); },
                    ')' | ']' | '}' | '>' => {
                        if let Some(last) = opened.pop() {
                            let expected = get_closing_character(&last).unwrap();
                            if c != expected {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    },
                    _ => { }
                }
            }
            return Some(opened);
        }

        let lines = input.lines();

        let incomplete_lines = lines.map(|l| find_incomplete_lines(l.to_string()) )
            .collect::<Vec<Option<Vec<char>>>>();

        let missing_chars = incomplete_lines.iter().filter(|l| l.is_some()).map(|l| {
            let chars = l.as_ref().unwrap();
            return chars.iter().rev().map(|c| {
                return get_closing_character(c).unwrap();
            }).collect::<Vec<char>>();
        }).collect::<Vec<Vec<char>>>();

        let mut scores = missing_chars.iter().map(|chars| {
            return chars.iter().fold(0, |acc, c| {
                return acc * 5 + match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0
                }
            })
        }).collect::<Vec<i64>>();
        scores.sort();

        return scores[scores.len()/2] as i64;
    }
}

fn main() {
    run(
        "Advent of Code day 10", 
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
    [({(<(())[]>[[{[]{<()<>>
    [(()[<>])]({[<{<<[]>>(
    {([(<{}[<>[]}>{[]{[(<()>
    (((({<>}<{<{<>}{[]{[]{}
    [[<[([]))<([[{}[[()]]]
    [{[{({}]{}}([{[{{{}}([]
    {<[[]]>}<{[{[{[]{()[[[]
    [<(<(<(<{}))><([]([]()
    <{([([[(<>()){}]>(<<{{
    <{([{{}}[<[[[<>{}]]]>[]]
    "});
    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 26397);
  }

  #[test]
  fn test_part2_1() {
    let input = String::from(indoc!{"
    [({(<(())[]>[[{[]{<()<>>
    [(()[<>])]({[<{<<[]>>(
    {([(<{}[<>[]}>{[]{[(<()>
    (((({<>}<{<{<>}{[]{[]{}
    [[<[([]))<([[{}[[()]]]
    [{[{({}]{}}([{[{{{}}([]
    {<[[]]>}<{[{[{[]{()[[[]
    [<(<(<(<{}))><([]([]()
    <{([([[(<>()){}]>(<<{{
    <{([{{}}[<[[[<>{}]]]>[]]
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 288957);
  }
}