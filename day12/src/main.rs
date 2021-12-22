use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use regex::Regex;

use aoc_utils::{AoCSolution, run};

struct Solution {}

fn is_lower_case(s: &str) -> bool {
  let re = Regex::new("^[a-z].*").unwrap();
  return re.is_match(s);
}

fn get_connection_pairs(input: &String) -> HashMap<&str, Vec<&str>> {
  let connections_pairs = input.lines().map(|l| {
    let splits: Vec<&str> = l.split('-').collect();
    return (splits[0],splits[1]);
  }).collect::<Vec<(&str,&str)>>();

  let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();

  for p in connections_pairs {
    let entry_to = connections.entry(p.0);
    match entry_to {
      Entry::Occupied(o) => { o.into_mut().push(p.1); },
      Entry::Vacant(v) => { v.insert(vec![p.1]); }
    }
    let entry_from = connections.entry(p.1);
    match entry_from {
      Entry::Occupied(o) => { o.into_mut().push(p.0); },
      Entry::Vacant(v) => { v.insert(vec![p.0]); }
    }
  }
  return connections;
}

impl AoCSolution for Solution  {
    fn part1(&self, input: &String) -> i64 {

        let connections = get_connection_pairs(input);

        fn find_paths(current: &str, mut previous: Vec<String>, connections_map: &HashMap<&str, std::vec::Vec<&str>>, paths: &mut Vec<Vec<String>>) {
          previous.push(current.to_string());

          // add to visited list
          match current {
            "end" => {
              paths.push(previous);
            },
            _ => {
              connections_map.get(current).map(|possible_connections| {
                for p in possible_connections {
                  if !is_lower_case(p) {
                    find_paths(p, previous.clone(), connections_map, paths);
                  } else if !previous.contains(&p.to_string()) {
                    find_paths(p, previous.clone(), connections_map, paths);
                  }
                  
                }
              });
            }
          }
        }

        let mut paths = vec![];
        find_paths("start", vec![], &connections, &mut paths);
    
        return paths.len() as i64;
    }
    
    fn part2(&self, input: &String) -> i64 {

        let connections = get_connection_pairs(input);

        fn get_hashmap_from_previous(previous: &Vec<String>) -> HashMap<&str,u8> {
          //create visited map
          let mut map : HashMap<&str,u8> = HashMap::new();
          for p in previous {
            if is_lower_case(p) {
              *map.entry(&p[..]).or_insert(0) += 1;
            } 
          }
          return map;
        }

        fn find_paths(current: &str, mut previous: Vec<String>, connections_map: &HashMap<&str, std::vec::Vec<&str>>, paths: &mut Vec<Vec<String>>) {
          previous.push(current.to_string());

          match current {
            "end" => {
              paths.push(previous);
            },
            _ => {
              connections_map.get(current).map(|possible_connections| {
                for p in possible_connections {
                  if !is_lower_case(p) {
                    find_paths(p, previous.clone(), connections_map, paths);
                  } else if p != &"start" {
                    let visited_map = get_hashmap_from_previous(&previous);
                    let visited_map_max = visited_map.iter().fold(0,|acc,(_,c)| *c.max(&acc));
                    let visits = *visited_map.get(*p).unwrap_or(&0);
                    if  visits < 1 || visited_map_max < 2 {
                      find_paths(p, previous.clone(), connections_map, paths);
                    }
                  }
                }
              });
            }
          }
        }

        let mut paths = vec![];
        find_paths("start", vec![], &connections, &mut paths);
    
        return paths.len() as i64;
    }
}

fn main() {
    run(
        "Advent of Code day 12", 
        &fs::read_to_string("input.txt").expect("Input Error"), 
        &Solution {}
    );
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  #[test]
  fn test_is_uppercase() {
    assert_eq!(is_lower_case("TEST"), false);
    assert_eq!(is_lower_case("test"), true);
  }

  #[test]
  fn test_part1_1() {
    let input = String::from(indoc!{"
    start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end
    "});
    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 10);
  }

  #[test]
  fn test_part1_2() {
    let input = String::from(indoc!{"
    dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc
    "});
    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 19);
  }

  #[test]
  fn test_part1_3() {
    let input = String::from(indoc!{"
    fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW
    "});
    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 226);
  }



  #[test]
  fn test_part2_1() {
    let input = String::from(indoc!{"
    start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 36);
  }

  #[test]
  fn test_part2_2() {
    let input = String::from(indoc!{"
    dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 103);
  }

  #[test]
  fn test_part2_3() {
    let input = String::from(indoc!{"
    fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 3509);
  }
}