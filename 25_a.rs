use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;




fn main() -> io::Result<()> {
  let filename = "25_in.txt";
  let file = File::open(filename.clone())?;
  let reader = BufReader::new(file);
  let mut this_line = String::new();

  let mut map = Vec::new();
  let mut new_map = Vec::new();





  let mut width = 0;
  let mut height = 0;


  for line in reader.lines() {
    height += 1;
    width = this_line.len();
    this_line = line?;
    let char_vec: Vec<char> = this_line.chars().collect();
    map.push(char_vec.clone());
    new_map.push(char_vec.clone());
  }

  //for x in &map {
  //  for c in x {
  //    print!("{}", c);
  //  }
  //  println!(" ");
  //}

  //println!("Width: {}", width);
  //println!("Height: {}", height);

  let mut steps = 0;

  // MAKE A MOVE

  while true {
  steps += 1;

  let mut did_i_move = 0;

  // Clear new_map
  for x in 0..width {
    for y in 0..height {
      new_map[y][x] = '.';
    }
  }


  // Move EAST
  for x in 0..width {
    for y in 0..height {
      if map[y][x] == 'v' {
        new_map[y][x] = 'v';
      }
      if map[y][x] == '>' {
        if x == width-1 {
          if map[y][0] == '.' {
            new_map[y][0] = '>';
            did_i_move = 1;
          } else {
            new_map[y][x] = '>';
          }
        } else {
          if map[y][x+1] == '.' {
            new_map[y][x+1] = '>';
            did_i_move = 1;
          } else {
            new_map[y][x] = '>';
          }
        }
      }
    }
  }

  // Copy map
  for x in 0..width {
    for y in 0..height {
      map[y][x] = new_map[y][x].clone();;
    }
  }

  // Clear new_map
  for x in 0..width {
    for y in 0..height {
      new_map[y][x] = '.';
    }
  }
 

  // Move SOUTH
  for x in 0..width {
    for y in 0..height {
      if map[y][x] == '>' {
        new_map[y][x] = '>';
      }
      if map[y][x] == 'v' {
        if y == height-1 {
          if map[0][x] == '.' {
            new_map[0][x] = 'v';
            did_i_move = 1;
          } else {
            new_map[y][x] = 'v';
          }
        } else {
          if map[y+1][x] == '.' {
            new_map[y+1][x] = 'v';
            did_i_move = 1;
          } else {
            new_map[y][x] = 'v';
          }
        }
      }
    }
  }

  // Copy map
  for x in 0..width {
    for y in 0..height {
      map[y][x] = new_map[y][x].clone();;
    }
  }

  // Print map
  //for x in &map {
  //  for c in x {
  //    print!("{}", c);
  //  }
  //  println!(" ");
  //}

  if did_i_move == 0 {
    break;
  }

  }

  println!("PART A: {}", steps);
  Ok(())
}


