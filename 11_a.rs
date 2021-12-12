use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let filename = "11_in.txt";
  let file = File::open(filename.clone())?;

  let reader = BufReader::new(file);
  let mut this_line = String::new();

  let mut array = vec![vec![0; 10]; 10];

  let mut depth = 0;
  for line in reader.lines() {
    this_line = line?;
    let mut width = 0;
    for c in this_line.chars() {
      array[depth][width] = c as i32 - 0x30;
      width += 1;
    }
    depth += 1;
  }




  let mut flashes = 0;
  let mut keep_going = 1;

  let steps = 200;
  let mut n = 0;
  while keep_going == 1 {
    n+=1;

    //Step 1
    for x in 0..10 {
      for y in 0..10 {
        array[y][x] += 1;
      }
    }

    // Step 2
    let mut flashed = 1;
    while flashed == 1 {
      flashed = 0;
      for x in 0..10 {
        for y in 0..10 {
          if array[y][x] > 9 {
            array[y][x] = -999;
            flashed = 1;
            if y > 0 && x > 0 {array[y-1][x-1] += 1;}
            if y > 0          {array[y-1][x]   += 1;}
            if y > 0 && x < 9 {array[y-1][x+1] += 1;}

            if          x > 0 {array[y][x-1]   += 1;}
            if          x < 9 {array[y][x+1]   += 1;}

            if y < 9 && x > 0 {array[y+1][x-1] += 1;}
            if y < 9          {array[y+1][x]   += 1;}
            if y < 9 && x < 9 {array[y+1][x+1] += 1;}
          }
        }
      }
    }
    let mut this_flashes = 0;
    for x in 0..10 {
      for y in 0..10 {
        if array[y][x] < 0 {
          array[y][x] = 0;
          if n <= 100 {
            flashes += 1;
          }
          this_flashes += 1;
        }
      }
    }
    if n == 100 {
      println!("PART A: {}", flashes);
    }

    if this_flashes == 100 {
      println!("PART B: {}", n);
      keep_going = 0;
    }



    //println!("--- {} -----------------------", n);
    //for row in array.clone() {
    //  println!("{} {} {} {} {} {} {} {} {} {}", row[0],row[1],row[2],row[3],row[4],row[5],row[6],row[7],row[8],row[9],);
    //}
  }

  Ok(())
}

