use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;


fn main() -> io::Result<()> {
  let filename = "15_in.txt";
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
  let mut result_array = vec![vec![0; width]; depth];


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

  for x in 0..depth {
    for y in 0..width {
      result_array[y][x] = 9999999;
    }
  }

  let mut start_deduct = array[0][0];

  let mut my_x = 0;
  let mut my_y = 0;

  make_move(&mut result_array, &array, my_x as usize, my_y as usize, 0, width-1, depth-1);

  println!("PART A: {}", result_array[depth-1][width-1]-start_deduct);
 

  Ok(())
}

fn make_move(res: &mut Vec<Vec<i32>>, m: &Vec<Vec<(i32)>>, m_x: usize, m_y: usize, mut steps: i32, width: usize, depth: usize) {
  let mut me_x = m_x;
  let mut me_y = m_y;
  //println!("Moving... x={} ; y={} ; width-{} ; depth={}", m_x, m_y, width, depth);

  steps += m[me_y][me_x];
  if steps >= res[me_y][me_x] {return;} else {res[me_y][me_x] = steps;}
  if me_x == width && me_y == depth {
    //println!("REACHED THE END in {} steps!", steps);
  } else {
    if me_x > 0 && m[me_y][me_x -1] > 0 { make_move(res, m, me_x-1, me_y, steps, width, depth); }
    if me_x < width && m[me_y][me_x +1] > 0 { make_move(res, m, me_x+1, me_y, steps, width, depth); }
    if me_y > 0 && m[me_y-1][me_x] > 0 { make_move(res, m, me_x, me_y-1, steps, width, depth); }
    if me_y < depth && m[me_y+1][me_x] > 0 { make_move(res, m, me_x, me_y+1, steps, width, depth); }
  }
}




  

