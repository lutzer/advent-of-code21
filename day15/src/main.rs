use std::fs;

use aoc_utils::{AoCSolution, run};

struct Solution {}

#[derive(Debug)]
struct Map {
    data: Vec<u32>,
    size: (usize,usize)
}

fn get_possible_paths(i: usize, w: usize, h: usize ) -> Vec<usize> {
  let (x,y) = (i % w, i / w); 
  let mut neighbours = vec![];
  if x > 0 {
    neighbours.push(i-1);
  }
  if x < w-1 {
    neighbours.push(i+1);
  }
  if y > 0 {
    neighbours.push(i-w);
  }
  if y < h-1 {
    neighbours.push(i+w);
  }
  
  return neighbours;
}

impl AoCSolution for Solution  {
    fn part1(&self, input: &String) -> i64 {

      let data = input.lines().fold(vec![],|mut acc, line| {
        for c in line.chars() {
            acc.push(c.to_string().parse::<u32>().unwrap());
        }
        return acc;
      });

      let width = input.lines().nth(1).unwrap().len();
      let height = data.len()/width;

      let map = Map {
          data: data,
          size: (width, height)
      };

      let end = map.data.len() - 1;
      let mut queue : Vec<(usize,u32)> = vec![];
      let mut visited = vec![false; map.data.len()];

      queue.push((0,0)); // i, risk

      while let Some((current_pos, current_risk)) = queue.pop() { 
        if current_pos == end {
          return current_risk as i64;
        } else if visited[current_pos] {
          continue;
        }

        visited[current_pos] = true;
        let neighbours = get_possible_paths(current_pos, map.size.0, map.size.1);
        for n in neighbours {
          let risk = current_risk + map.data[n];
          if !visited[n] {
            queue.push((n,risk));
          }
        }
        queue.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap() );
      }

      return 0 as i64;
    }
    
    fn part2(&self, input: &String) -> i64 {

      const MAP_EXPANSION: usize = 5;

      let data = input.lines().fold(vec![],|mut acc, line| {
        for c in line.chars() {
            acc.push(c.to_string().parse::<u32>().unwrap());
        }
        return acc;
      });

      let width = input.lines().nth(1).unwrap().len();
      let height = data.len()/width;

      let map = Map {
          data: data,
          size: (width, height)
      };

      fn get_map_value(i: usize, map: &Map, expand: usize) -> u32 {
        let expanded_size = (map.size.0*expand, map.size.1*expand);
        let (x,y) = (i % expanded_size.0, i / expanded_size.1); 
        let j = x % map.size.0 + y % map.size.1 * map.size.0;
        let add = (x/map.size.0 + y/map.size.1) as u32;
        return (map.data[j] + add - 1) % 9 + 1;
      }

     
      // fn print_map(map: &Map, expand: usize) {
      //   for i in 0..map.data.len()*expand*expand {
      //     print!("{}.", get_map_value(i, &map, 5));
      //     if i % (map.size.0*expand) == (map.size.0*expand-1) {
      //       println!();
      //     }
      //   }
      // }

      // let map2 = Map {
      //   data: vec![8],
      //   size: (1,1)
      // };

      // print_map(&map,5);
      

      let end = map.size.0 * map.size.1 * MAP_EXPANSION * MAP_EXPANSION - 1;
      let mut queue : Vec<(usize,u32)> = vec![];
      let mut visited = vec![false; map.data.len()*25];

      queue.push((0,0)); // i, risk

      while let Some((current_pos, current_risk)) = queue.pop() { 
        if current_pos == end {
          return current_risk as i64;
        } else if visited[current_pos] {
          continue;
        }

        visited[current_pos] = true;
        let neighbours = get_possible_paths(current_pos, map.size.0*MAP_EXPANSION, map.size.1*MAP_EXPANSION);
        for n in neighbours {
          let risk = current_risk + get_map_value(n, &map, MAP_EXPANSION);
          if !visited[n] {
            queue.push((n,risk));
          }
        }
        queue.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap() );
      }


      return 0 as i64;
    }
}

fn main() {
    run(
        "Advent of Code day 15", 
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
    1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581
    "});
    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 40);
  }

  #[test]
  fn test_part2_1() {
    let input = String::from(indoc!{"
    1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 315);
  }

}