use std::fs;
use regex::Regex;

use aoc_utils::{AoCSolution, run};

struct Solution {}

#[derive(Debug)]
struct Target {
  x : (i32,i32),
  y: (i32, i32)
}

impl Target {

  fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Target {
    let x = (x1.min(x2), x1.max(x2));
    let y = (y1.min(y2), y1.max(y2));
    return Target { x, y };
  }
}

type Vector2D = (i32,i32);

fn is_target_hit(velocity : Vector2D, target: &Target) -> bool {
  let mut current_pos : Vector2D = (0,0);
  let mut current_velocity = velocity;
  loop {
    if current_pos.1 < target.y.0 && current_velocity.1 < 0 {
      return false;
    } else if current_pos.1 <= target.y.1 && current_pos.1 >= target.y.0 && current_pos.0 >= target.x.0 && current_pos.0 <= target.x.1 {
      return true;
    }
    current_pos.0 += current_velocity.0;
    current_pos.1 += current_velocity.1;
    current_velocity.0 = 0.max(current_velocity.0-1);
    current_velocity.1 -= 1;
  }
}

fn is_target_hit_y(velocity : Vector2D, target: &Target) -> (bool,i32) {
  let mut current_pos : Vector2D = (0,0);
  let mut current_velocity = velocity;
  let mut max_y = 0;
  loop {
    max_y = current_pos.1.max(max_y);
    if current_pos.1 < target.y.0 && current_velocity.1 < 0 {
      return (false, max_y);
    } else if current_pos.1 <= target.y.1 && current_velocity.1 < 0 {
      return (true, max_y);
    }
    current_pos.1 += current_velocity.1;
    current_velocity.1 -= 1;
  }
}

impl AoCSolution for Solution  {
    fn part1(&self, input: &String) -> i64 {
      let re = Regex::new(r".+x=(\-*\d+)..(\-*\d+), y=(\-*\d+)..(\-*\d+)").unwrap();
      let cap = re.captures(input).unwrap();
      let target = Target::new(
        cap[1].parse::<i32>().unwrap(), cap[2].parse::<i32>().unwrap(),
        cap[3].parse::<i32>().unwrap(), cap[4].parse::<i32>().unwrap()
      );
      
      let mut max_y = 0;
      for vel_y in 1..1000 {
        let (hit,y) = is_target_hit_y((0,vel_y), &target);
        max_y = if hit {y} else {max_y};
      }

      println!("{:?}", target);
      return max_y as i64;
    }
    
    fn part2(&self, input: &String) -> i64 { 
      let re = Regex::new(r".+x=(\-*\d+)..(\-*\d+), y=(\-*\d+)..(\-*\d+)").unwrap();
      let cap = re.captures(input).unwrap();
      let target = Target::new(
        cap[1].parse::<i32>().unwrap(), cap[2].parse::<i32>().unwrap(),
        cap[3].parse::<i32>().unwrap(), cap[4].parse::<i32>().unwrap()
      );

      let mut hits = 0;
      for vel_y in -500..500 {
        let (hit,_) = is_target_hit_y((0,vel_y), &target);
        if hit {
          for vel_x in -500..500 {
            if is_target_hit((vel_x, vel_y), &target) {
              hits +=1;
            }
          }
        }
      }

      return hits as i64;
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
  fn test_is_target_hit() {
    let target = Target::new(20,30,-10,-5);
    assert_eq!(is_target_hit((7,2), &target), true);
    assert_eq!(is_target_hit((6,3), &target), true);
    assert_eq!(is_target_hit((9,0), &target), true);
    assert_eq!(is_target_hit((17,-4), &target), false);
  }

  #[test]
  fn test_part1_1() {
    let input = String::from(indoc!{"
    target area: x=20..30, y=-10..-5
    "});
    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 45);
  }

  #[test]
  fn test_part2_1() {
    let input = String::from(indoc!{"
    target area: x=20..30, y=-10..-5
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 112);
  }

}