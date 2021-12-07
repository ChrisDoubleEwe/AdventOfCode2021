use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let file = File::open("05_in.txt")?;
  let reader = BufReader::new(file);

  let width = 1000;
  let height = 1000;

  let mut array = vec![vec![0; width]; height];

  for line in reader.lines() {
    let this_line = line?;
    let split = this_line.split(" -> ");
    //println!("{}", this_line);

    let pair1 = split.clone().nth(0).unwrap().split(",");
    let pair2 = split.clone().nth(1).unwrap().split(",");
    let mut from_x = pair1.clone().nth(0).unwrap().parse::<i32>().unwrap();
    let mut from_y = pair1.clone().nth(1).unwrap().parse::<i32>().unwrap();
    let mut to_x = pair2.clone().nth(0).unwrap().parse::<i32>().unwrap();
    let mut to_y = pair2.clone().nth(1).unwrap().parse::<i32>().unwrap();

    if from_x > to_x {
      let temp_x = from_x;
      from_x = to_x;
      to_x = temp_x;
    }

    if from_y > to_y {
      let temp_y = from_y;
      from_y = to_y;
      to_y = temp_y;
    }


    if from_x == to_x {
      for y in from_y..to_y+1 {
        array[y as usize][from_x as usize] += 1;
      }
    } else if from_y == to_y {
      for x in from_x..to_x+1 {
        array[from_y as usize][x as usize] += 1;

      }
    }
    //for a in array.clone() {
    //  println!("{:?}", a);
    //}
 
  }

  let mut count = 0;
  for a in array {
    for i in a {
      if i > 1 {
        count += 1;
      }
    }
  }
  println!("Part A: {}", count);
  Ok(())
}

