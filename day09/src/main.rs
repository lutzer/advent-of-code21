use std::fs;

use aoc_utils::{AoCSolution, run};

struct Solution {}

#[derive(Debug)]
struct HeightMap {
    data: Vec<u8>,
    size: (usize,usize)
}

fn get_neighbours(i: usize, w: usize, h: usize ) -> Vec<usize> {
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
                acc.push(c.to_string().parse::<u8>().unwrap());
            }
            return acc;
        });

        let width = input.lines().nth(1).unwrap().len();
        let height = data.len()/width;

        let heightmap = HeightMap {
            data: data,
            size: (width, height)
        };

        let low_points = heightmap.data.iter().enumerate().filter(|(i,&h)| {
            let neigbours = get_neighbours(*i, width, height);
            return !neigbours.iter().any(|&i| {
                heightmap.data[i] <= h
            });
        }).collect::<Vec<(usize, &u8)>>();
        
        let sum : i64 = low_points.iter().fold(0,|acc,p| acc + 1 + *p.1 as i64);

        return sum;
    }
    
    fn part2(&self, input: &String) -> i64 {

        let data = input.lines().fold(vec![],|mut acc, line| {
            for c in line.chars() {
                acc.push(c.to_string().parse::<u8>().unwrap());
            }
            return acc;
        });

        let width = input.lines().nth(1).unwrap().len();
        let height = data.len()/width;

        let heightmap = HeightMap {
            data: data,
            size: (width, height)
        };

        let low_points = heightmap.data.iter().enumerate().filter(|(i,&h)| {
            let neigbours = get_neighbours(*i, width, height);
            return !neigbours.iter().any(|&i| {
                heightmap.data[i] <= h
            });
        }).collect::<Vec<(usize, &u8)>>();

        let basins = low_points.iter().map(|(lp,_)| {
            let mut visited = vec![false; heightmap.data.len()];
            let mut basin_locations = vec![];

            let mut queue = vec![*lp];
            visited[*lp] = true;

            while let Some(i) = queue.pop() {
                if heightmap.data[i] >= 9 {
                    continue;
                }
                basin_locations.push((i,heightmap.data[i]));
                let neighbours = get_neighbours(i, heightmap.size.0, heightmap.size.1);
                for n in neighbours {
                    if !visited[n] {
                        visited[n] = true;
                        queue.push(n);
                    }
                }
            }
            return basin_locations;
        }).collect::<Vec<Vec<(usize,u8)>>>();

        let mut basin_sizes = basins.iter().map(|b| b.len()).collect::<Vec<usize>>();
        basin_sizes.sort();

        let product = basin_sizes.iter().rev().take(3).fold(1,|acc,b| acc * b);

        return product as i64;
    }
}

fn main() {
    run(
        "Advent of Code day 09", 
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
    2199943210
    3987894921
    9856789892
    8767896789
    9899965678
    "});
    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 15);
  }

  #[test]
  fn test_part2_1() {
    let input = String::from(indoc!{"
    2199943210
    3987894921
    9856789892
    8767896789
    9899965678
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 1134);
  }
}