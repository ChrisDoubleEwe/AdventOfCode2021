use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let filename = "09_in.txt";
  let file = File::open(filename.clone())?;
  let file2 = File::open(filename.clone())?;


  let reader = BufReader::new(file);
  let mut width = 0;
  let mut depth = 0;


  let mut this_line = String::new();

  for line in reader.lines() {
    this_line = line?;
    width = this_line.len();
    depth += 1;
  }
  let mut array = vec![vec![0; width]; depth];

  depth = 0;

  let reader2 = BufReader::new(file2);

  for line in reader2.lines() {
    this_line = line?;
    width = 0;
    for c in this_line.chars() {
      array[depth][width] = c as i32 - 0x30;
      width += 1;
    }
    depth += 1;
  }

  // array[y][x]
  //for i in 0..depth {
  //  println!("{:?}", array[i]);
  //}

  let mut parta = 0;
  for y in 0..depth {
    for x in 0..width {
      let mut lowest = 1;
      if y > 0 {
        if array[y][x] >= array[y-1][x] { lowest = 0}
      }
      if y < depth-1 {
        if array[y][x] >= array[y+1][x] { lowest = 0}
      }
      if x > 0 {
        if array[y][x] >= array[y][x-1] { lowest = 0}
      }
      if x < width-1 {
        if array[y][x] >= array[y][x+1] { lowest = 0}
      }
      if lowest == 1 {
        //println!("LOWEST: {}", array[y][x]);
        parta += 1;
        parta += array[y][x];
      }
    }
  }


  println!("PART A: {}", parta);

  Ok(())
}

