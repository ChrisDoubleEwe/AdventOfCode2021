use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let filename = "13_in.txt";
  let file = File::open(filename.clone())?;
  let file2 = File::open(filename.clone())?;


  let reader = BufReader::new(file);
  let reader2 = BufReader::new(file2);

  let mut width = 0;
  let mut depth = 0;

  let mut folds = Vec::new();
  let mut pair = ('z', 0);



  let mut this_line = String::new();

  let mut max_x = 0;
  let mut max_y = 0;

  for line in reader.lines() {
    this_line = line?;
    if this_line.contains(',') {
      let x = this_line.split(",").nth(0).unwrap().parse::<i32>().unwrap();
      if x > max_x {
        max_x = x;
      }
      let y = this_line.split(",").nth(1).unwrap().parse::<i32>().unwrap();
      if y > max_y {
        max_y = y;
      }
    }
    if this_line.contains("fold ") {
      let f = this_line.split(" along ").nth(1).unwrap();
      let dir = f.split('=').nth(0).unwrap();
      let dist = f.split('=').nth(1).unwrap().parse::<i32>().unwrap();
      pair.0 = dir.chars().nth(0).unwrap();
      pair.1 = dist;
      folds.push(pair);
    }


  }

  max_x += 1;
  let maxxx_x = max_x + 10;
  max_y += 1;
  let maxxx_y = max_y + 10;
  let mut array = vec![vec![0; maxxx_x as usize]; maxxx_y as usize];

  for line in reader2.lines() {
    this_line = line?;
    if this_line.contains(',') {
      let x = this_line.split(",").nth(0).unwrap().parse::<i32>().unwrap();
      let y = this_line.split(",").nth(1).unwrap().parse::<i32>().unwrap();
      array[y as usize][x as usize] = 1;
    }
  }







  let mut first = 1;
  for f in folds {
    if first == 1 {

      if f.0 == 'x' {
        // FOLD ALONG X, ie fold cols left
        for col in 0..f.1+1 {
          for y in 0..max_y {
            let fold_from = f.1 + col;
            let fold_to = f.1 - col;
            if array[y as usize][fold_from as usize] == 1 {
              array[y as usize][fold_to as usize] = array[y as usize][fold_from as usize];
            }
            array[y as usize][fold_from as usize] = 0;
          }
        }

      } else {
        // FOLD ALONG Y, ie fold rows up
        for row in 0..f.1+1 {
          for x in 0..max_x {
            let fold_from = f.1 + row;
            let fold_to = f.1 - row;
            if array[fold_from as usize][x as usize] == 1 {
              array[fold_to as usize][x as usize] = array[fold_from as usize][x as usize];
            } 
            array[fold_from as usize][x as usize] = 0;
          }
        }

      }

    }
    //first = 0;
  }

  let mut max_rows = 0;
  let mut max_cols = 0;
 
  let mut num_rows = 0;
  for row in array.clone() {
    num_rows += 1;
    let mut col = 0;
    for c in row {
      if c == 1 {
        max_cols = col+1;
        max_rows = num_rows;
      }
      col += 1;
    }
  }

  for row in 0..max_rows {
    let mut out = "".to_string();
    for c in 0..max_cols+20 {
      if array[row][c] == 1 {
        out += "#";
      } else {
        out += " ";
      }
    }
    println!("{}", out);
  }


  Ok(())
}

