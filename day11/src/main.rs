use std::fs;

use aoc_utils::{AoCSolution, run};

struct Solution {}

fn print_map(data: &Vec<u8>, w: usize) {
    for (i,d) in data.iter().enumerate() {
        print!("{}", d);
        if i % w == w-1 {
            println!();
        }
    }
    println!("---");
}

fn get_neighbours(i: usize, w: usize, h: usize ) -> Vec<usize> {
    let (x,y) = (i % w, i / w); 
    let mut neighbours = vec![];
    // horizontally
    if x > 0 {
        neighbours.push(i-1);
        // diagonally
        if y > 0 {
            neighbours.push(i-w-1);
        }
        if y < h -1 {
            neighbours.push(i+w-1);
        }
    }
    if x < w-1 {
        neighbours.push(i+1);
        // diagonally
        if y > 0 {
            neighbours.push(i-w+1);
        }
        if y < h -1 {
            neighbours.push(i+w+1);
        }
    }
    // vertically
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

        let mut octopus_map = input.lines().fold(vec![],|mut acc, line| {
            for c in line.chars() {
                acc.push(c.to_string().parse::<u8>().unwrap());
            }
            return acc;
        });

        let mut flash_count = 0;

        for _ in 0..100 {
            octopus_map = octopus_map.iter().map(|o| o + 1).collect();

            let mut flash_queue = octopus_map.iter().clone().enumerate().filter(|(_,o)| **o > 9)
                .map(|(i,_)| i).collect::<Vec<usize>>();

            let mut flashed = vec![];

            while let Some(i) = flash_queue.pop() {
                if flashed.contains(&i) {
                    continue;
                }
                flashed.push(i);
                let neighbours = get_neighbours(i, 10, 10);
                for j in neighbours {
                    octopus_map[j] += 1;
                    if octopus_map[j] > 9 {
                        flash_queue.push(j);
                    }
                }
            }

            octopus_map = octopus_map.iter().map(|x| if *x > 9 { 0 } else { *x }).collect();
            flash_count += flashed.len();
        }

        return flash_count as i64;
    }
    
    fn part2(&self, input: &String) -> i64 {
        let mut octopus_map = input.lines().fold(vec![],|mut acc, line| {
            for c in line.chars() {
                acc.push(c.to_string().parse::<u8>().unwrap());
            }
            return acc;
        });

        let mut step = 0;

        loop {
            octopus_map = octopus_map.iter().map(|o| o + 1).collect();

            let mut flash_queue = octopus_map.iter().clone().enumerate().filter(|(_,o)| **o > 9)
                .map(|(i,_)| i).collect::<Vec<usize>>();

            let mut flashed = vec![];

            while let Some(i) = flash_queue.pop() {
                if flashed.contains(&i) {
                    continue;
                }
                flashed.push(i);
                let neighbours = get_neighbours(i, 10, 10);
                for j in neighbours {
                    octopus_map[j] += 1;
                    if octopus_map[j] > 9 {
                        flash_queue.push(j);
                    }
                }
            }

            octopus_map = octopus_map.iter().map(|x| if *x > 9 { 0 } else { *x }).collect();
            
            step += 1;

            if flashed.len() == octopus_map.len() {
                break;
            }
        }

        return step as i64;
    }
}

fn main() {
    run(
        "Advent of Code day 11", 
        &fs::read_to_string("input.txt").expect("Input Error"), 
        &Solution {}
    );
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  #[test]
  fn test_get_neighbours() {
    let mut n1 = get_neighbours(0, 10, 10);
    n1.sort();
    assert_eq!(n1, vec![1,10,11]);
    let mut n2 = get_neighbours(1, 10, 10);
    n2.sort();
    assert_eq!(n2, vec![0,2,10,11,12]);
    let mut n3 = get_neighbours(11, 10, 10);
    n3.sort();
    assert_eq!(n3, vec![0,1,2,10,12,20,21,22]);
    let mut n4 = get_neighbours(99, 10, 10);
    n4.sort();
    assert_eq!(n4, vec![88,89,98]);

  }

  #[test]
  fn test_part1_1() {
    let input = String::from(indoc!{"
    5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526
    "});
    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 1656);
  }

  #[test]
  fn test_part2_1() {
    let input = String::from(indoc!{"
    5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 195);
  }
}