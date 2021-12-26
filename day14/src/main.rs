use std::collections::HashMap;
use std::fs;

use aoc_utils::{AoCSolution, run};

struct Solution {}

fn get_string_pairs(s: &String) -> Vec<String> {
  let mut pairs = vec![];
  for i in 0..s.len()-1 {
    pairs.push(s[i..i+2].to_string());
  }
  return pairs;
} 

impl AoCSolution for Solution  {
    fn part1(&self, input: &String) -> i64 {

      let mut template = input.lines().nth(0).unwrap().to_string();

      let insertions: HashMap<&str,&str> = input.lines().skip(2).map(|l| {
        let splits: Vec<&str> = l.split(" -> ").collect();
        return (splits[0], splits[1]);
      }).fold(HashMap::new(), |mut acc, (from,to)| {
        acc.insert(from,to);
        return acc;
      });

      for _ in 0..10 {
        let template_pairs = get_string_pairs(&template);
        let mut new_insertions = vec![];
        for (i,tp) in template_pairs.iter().enumerate() {
          if insertions.contains_key(&tp[..]) {
            new_insertions.push((i+1, insertions.get(&tp[..]).unwrap()));
          } 
        }
        for (i,(insert_index, insert)) in new_insertions.iter().enumerate() {
          template.insert_str(*insert_index+i, insert);
        }
      }

      let count_elements: HashMap<char,u32> = template.chars().fold(HashMap::new(),|mut acc, c| { 
        *acc.entry(c).or_insert(0) += 1;
        return acc 
      });

      let mut sorted_count_elements : Vec<(char, u32)> = count_elements.into_iter().collect();
      sorted_count_elements.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

      return ( sorted_count_elements[sorted_count_elements.len()-1].1 - sorted_count_elements[0].1) as i64;
    }
    
    fn part2(&self, input: &String) -> i64 {

      let template = input.lines().nth(0).unwrap().to_string();

      let insertions: HashMap<&str,&str> = input.lines().skip(2).map(|l| {
        let splits: Vec<&str> = l.split(" -> ").collect();
        return (splits[0], splits[1]);
      }).fold(HashMap::new(), |mut acc, (from,to)| {
        acc.insert(from,to);
        return acc;
      });


      let mut pair_counts: HashMap<String,i64> = get_string_pairs(&template).into_iter().fold(HashMap::new(), |mut acc, p| {
        *acc.entry(p).or_insert(0) += 1;
        return acc;
      });

      for _ in 0..40 {
        let mut add_pairs: HashMap<String,i64> = HashMap::new();
        for (insert_pair, insert) in &insertions {
          // if pair_counts.contains_key(&insert_pair.to_string()) {
            if let Some(p) = pair_counts.get(&insert_pair.to_string()) {
            let pair1 = insert_pair.chars().nth(0).unwrap().to_string() + insert;
            let pair2 = insert.to_string() + &insert_pair.chars().nth(1).unwrap().to_string()[..];
            *add_pairs.entry(insert_pair.to_string()).or_insert(0) -= p;
            *add_pairs.entry(pair1).or_insert(0) += p;
            *add_pairs.entry(pair2).or_insert(0) += p;
          }
        }
        for (p,c) in add_pairs {
          *pair_counts.entry(p).or_insert(0) += c;
        }
      }

      let mut count_elements: HashMap<char,i64> = pair_counts.iter().fold(HashMap::new(),|mut acc, p| {
        let char1 = p.0.chars().nth(0).unwrap();
        *acc.entry(char1).or_insert(0) += p.1;
        return acc 
      });
      // add one for the last character of template
      *count_elements.entry(template.chars().last().unwrap()).or_insert(0) += 1;
      
      let mut sorted_count_elements : Vec<(char, i64)> = count_elements.into_iter().collect();
      sorted_count_elements.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

      return ( sorted_count_elements[sorted_count_elements.len()-1].1 - sorted_count_elements[0].1) as i64;
    }
}

fn main() {
    run(
        "Advent of Code day 14", 
        &fs::read_to_string("input.txt").expect("Input Error"), 
        &Solution {}
    );
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  #[test]
  fn test_get_string_pairs() {
    let pairs = get_string_pairs(&"NABE".to_string());
    assert_eq!(pairs, vec!["NA","AB","BE"]);
  }


  #[test]
  fn test_part1_1() {
    let input = String::from(indoc!{"
    NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C
    "});
    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 1588);
  }

  #[test]
  fn test_part2_1() {
    let input = String::from(indoc!{"
    NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 2188189693529);
  }

}