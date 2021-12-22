use clap::{Arg, App};

pub fn run(title: &str, input: &String, s: &dyn AoCSolution) {
  let args = App::new(title)
    .arg(Arg::with_name("part")
      .takes_value(true)
      .required(true))
    .get_matches();

  let part = args.value_of("part").unwrap_or("").to_string();

  let result = match part.as_str() {
      "1" => { s.part1(&input) },
      "2" => { s.part2(&input) },
      _ => { panic!("Select either part 1 or 2") }
  };
  println!("Result is: {}", result);
}

pub trait AoCSolution {
  fn part1(&self, input: &String) -> i64;
  fn part2(&self, input: &String) -> i64;
}