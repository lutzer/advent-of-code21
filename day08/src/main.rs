use std::fs;

use aoc_utils::{AoCSolution, run};

struct Solution {}

#[derive(Debug)]
struct Pattern<'a> {
    signals: Vec<&'a str>,
    outputs: Vec<&'a str>
}

impl AoCSolution for Solution  {
    fn part1(&self, input: &String) -> i64 {

        let patterns = input.lines().map(|l| {
            let splits: Vec<&str> = l.split(" | ").collect();
            return Pattern {
                signals: splits[0].split(" ").collect(),
                outputs: splits[1].split(" ").collect()
            };
        }).collect::<Vec<Pattern>>();

        // 1 = 2, 4 = 4, 7 = 3, 8 = 7 (digits = segments)
        let count = patterns.iter().fold(0, |acc, p| {
            return acc + p.outputs.iter().fold(0, |acc, x| {
                return acc + if x.len() == 2 || x.len() == 4 || x.len() == 3 || x.len() == 7 { 1 } else { 0 }
            })
        });
        
        return count;
    }
    
    fn part2(&self, input: &String) -> i64 {

        let patterns = input.lines().map(|l| {
            let splits: Vec<&str> = l.split(" | ").collect();
            return Pattern {
                signals: splits[0].split(" ").collect(),
                outputs: splits[1].split(" ").collect()
            };
        }).collect::<Vec<Pattern>>();

        fn contains_chars(input: &str, chars: &str) -> bool {
            for c in chars.chars() {
                if !input.contains(c) {
                    return false
                }
            }
            return true;
        }

        fn has_same_chars(str1: &str, str2: &str) -> bool {
            if str1.len() != str2.len() {
                return false
            }
            let mut chars1 : Vec<char> = str1.chars().collect();
            let mut chars2 : Vec<char> = str2.chars().collect();
            chars1.sort();
            chars2.sort();
            for (i,c) in chars1.iter().enumerate() {
                if *c != chars2[i] { return false }
            }
            return true;
        }

        // 1 = 2, 4 = 4, 7 = 3, 8 = 7 (digits = segments)
        fn decode_signals(signals: &Vec<&str>) -> Vec<String> {
            let mut digits : Vec<String>  = vec![String::from("");10];
            digits[1] = signals.iter().find(|x| x.len() == 2).unwrap().to_string();
            digits[4] = signals.iter().find(|x| x.len() == 4).unwrap().to_string();
            digits[7] = signals.iter().find(|x| x.len() == 3).unwrap().to_string();
            digits[8] = signals.iter().find(|x| x.len() == 7).unwrap().to_string();
            
            // find 6 & 9 & 0
            let mut iter = signals.iter();
            let mut candidates_690 = vec![];
            candidates_690.push(iter.find(|x| x.len() == 6).unwrap().to_string());
            candidates_690.push(iter.find(|x| x.len() == 6).unwrap().to_string());
            candidates_690.push(iter.find(|x| x.len() == 6).unwrap().to_string());
            // find 9
            candidates_690.retain(|c| {
                if contains_chars(&*c, &*digits[4]) {
                    digits[9] = c.clone();
                    return false;
                }
                return true;
            });
            // find 0
            candidates_690.retain(|c| {
                if contains_chars(&*c, &*digits[1]) {
                    digits[0] = c.clone();
                    return false;
                }
                return true;
            });
            digits[6] = candidates_690[0].clone();

            // find 2,3,5
            let mut iter = signals.iter();
            let mut candidates_235 = vec![];
            candidates_235.push(iter.find(|x| x.len() == 5).unwrap().to_string());
            candidates_235.push(iter.find(|x| x.len() == 5).unwrap().to_string());
            candidates_235.push(iter.find(|x| x.len() == 5).unwrap().to_string());

            // find 3
            candidates_235.retain(|c| {
                if contains_chars(&*c, &*digits[1]) {
                    digits[3] = c.clone();
                    return false;
                }
                return true;
            });
            // find 5
            candidates_235.retain(|c| {
                if contains_chars(&*digits[9], &*c) {
                    digits[5] = c.clone();
                    return false;
                }
                return true;
            });
            // find 2
            digits[2] = candidates_235[0].clone();

            return digits;
        }

        fn decode_output(output: &Vec<&str>, digits: &Vec<String>) -> u32 {
            let mut sum = 0;
            for (i,&o) in output.iter().rev().enumerate() {
                let digit = digits.iter().position(|x| { 
                    return has_same_chars(x,o);
                }).unwrap() as u32;
                sum += digit * (10 as u32).pow(i as u32);
            }
            return sum;
        }

        let sum = patterns.iter().fold(0,|acc,p| {
            let digits = decode_signals(&p.signals);
            return acc + decode_output(&p.outputs, &digits);
        });

        return sum as i64;
    }
}

fn main() {
    run(
        "Advent of Code day 08", 
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
    be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "});

    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 26);
  }

  #[test]
  fn test_part_2_1() {
    let input = String::from(indoc!{"
    acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 5353);
  }

  #[test]
  fn test_part_2_2() {
    let input = String::from(indoc!{"
    be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 61229);
  }
}