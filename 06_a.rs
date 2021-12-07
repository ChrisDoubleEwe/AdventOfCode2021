use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let file = File::open("06_in.txt")?;
  let reader = BufReader::new(file);

  let mut fish = vec![0; 9];

  let mut this_line = String::new();
  for line in reader.lines() {
    this_line = line?;
  }

  let split = this_line.split(",");
  for s in split {
    let f = s.parse::<i64>().unwrap();
    fish[f as usize] += 1;
  }

  let mut total_fish: u64 = 0;

  for day in 1..257 {
    //println!("-----------");
    //println!("Day {} ", day);
    //println!("{:?} ", fish);

    // All the "0" fish become 6 and spawn an 8
    let num_zero = fish[0];

    // All the counts decrement by 1
    for f in 0..8 {
      fish[f as usize] = fish[f+1 as usize]
    }

    // Add the new 6 fish
    fish[6] += num_zero.clone();
    //println!("  creating {} 6 fish", num_zero.clone());

    // Add the new 8 fish
    fish[8] = num_zero.clone();
    //println!("  creating {} 8 fish", num_zero.clone());



    total_fish = 0;
    for f in 0..9 {
      total_fish += fish[f as usize]
    }

    if day == 80 {
      println!("Part A: {}", total_fish);
    }

    if day == 256 {
      println!("Part B: {}", total_fish);
    }


    //println!("Day {} : number of fish = {}", day, total_fish);
    //println!("{:?} ", fish);

  }




  Ok(())
}

