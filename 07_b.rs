use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let file = File::open("07_in.txt")?;
  let reader = BufReader::new(file);

  let mut min = 10000;
  let mut max = 0;


  let mut this_line = String::new();
  for line in reader.lines() {
    this_line = line?;
  }

  let mut crabs = Vec::new();

  let split = this_line.split(",");
  for s in split {
    let f = s.parse::<i32>().unwrap();
    if f < min {
      min = f;
    }
    if f > max {
      max = f;
    }
    crabs.push(f);
  }

  let mut min_fuel = 99999999;
  let mut min_pos = 999999999;
  for n in min..max+1 {
    let mut moves = 0;
    println!("Consider move to position {}", n);

    for c in crabs.clone() {
      let diff = c - n;
      moves += fuel(diff.abs());
    }
    if moves < min_fuel {
      min_fuel = moves.clone();
      min_pos = n.clone();
    }
    println!(" ... takes {} moves", moves);
  }

  println!("Part B: move to position {} takes {} fuel", min_pos, min_fuel);

  Ok(())
}

fn fuel (x: i32) -> i32{
  let mut result = 0;
  for i in 0..x+1 {
    result += i;
  }
  return result;
}
