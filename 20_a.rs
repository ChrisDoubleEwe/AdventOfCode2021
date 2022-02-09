use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;




fn main() -> io::Result<()> {
  let mut space = Vec::new();
  let mut pattern = Vec::new();


  let filename = "20_in.txt";
  let file = File::open(filename.clone())?;

  let reader = BufReader::new(file);

  let mut this_line = String::new();

  let mut first = 1;
  for line in reader.lines() {
    let mut line_vec = Vec::new();
    this_line = line?;

    if this_line.len() < 1 {
      continue;
    }

    if first == 1 {
      for c in this_line.chars() {
        pattern.push(c);
      }
      first = 0;
      continue;
    }

    for c in this_line.chars() {
      line_vec.push(c);
    }
    space.push(line_vec.clone());
  }

  //println!("PATTERN:");
  //println!("{:?}", pattern);


  let mut iterations = 2;
  let mut new_char = '.';


  println!(" ---- START --------");

  //for line in space.clone() {
  //  let mut out = String::new();
  //  for c in line {
  //    out += &c.to_string();
  //  }
  //  println!("{:?}", out);
  //}


  for i in 1..iterations+1 {
    println!(" ---- Iteration: {} --------", i);



    // Increase space if necessary
    let mut min_x = space[0].clone().len();
    let mut max_x = 0;
    let mut min_y = space.clone().len();
    let mut max_y = 0;

    for n in 0..space.len() {
      if space[n].contains(&'#') {
        if n < min_y { min_y = n; }
        if n > max_y { max_y = n; }
        let first_index = space[n].iter().position(|&r| r == '#').unwrap();
        let last_index = space[n].iter().rposition(|&r| r == '#').unwrap();

        if first_index < min_x {min_x = first_index; }
        if last_index > max_x {max_x = last_index; }
      }

    }

    let mut y_diff = space.len() - max_y;
    let mut x_diff = space[0].len() - max_x;


    //println!("MinY: {} ; MaxY: {} ; y_diff: {} ;  MinX: {} ; MaxX: {} ; x_diff: {}", min_y, max_y, y_diff, min_x, max_x, x_diff);

  
    let mut new_space = Vec::new();
    let mut new_empty_row = Vec::new();
    for c in space[0].clone() {
      new_empty_row.push(new_char.clone());
    }


    // ADD ROWS IF NEEDED...
    if min_y < 2 {
      new_space.push(new_empty_row.clone());
      new_space.push(new_empty_row.clone());
      new_space.push(new_empty_row.clone());
      new_space.push(new_empty_row.clone());

    }
    for line in space.clone() {
      new_space.push(line);
    }
    if y_diff <= 2 {
      new_space.push(new_empty_row.clone());
      new_space.push(new_empty_row.clone());
      new_space.push(new_empty_row.clone());
      new_space.push(new_empty_row.clone());

    }

    space = new_space.clone();

    // ADD COLUMNS IF NEEDED
  
    let mut new_new_space = Vec::new();
    for line in space.clone() {
      let mut new_line = Vec::new();
      if min_x < 2 {
        new_line.push(new_char.clone());
        new_line.push(new_char.clone());
        new_line.push(new_char.clone());
        new_line.push(new_char.clone());

      }
      for c in line.clone() {
        new_line.push(c);
      }
      if x_diff <= 2 {
        new_line.push(new_char.clone());
        new_line.push(new_char.clone());
        new_line.push(new_char.clone());
        new_line.push(new_char.clone());
      } 
      new_new_space.push(new_line.clone());
    }

    space = new_new_space.clone();




    // PROCESS IMAGE
    let mut new_space = space.clone();

    for x in 1..space[0].len()-1 {
      for y in 1..space.len()-1 {
        let mut binary_value = String::new();

        if space[y-1][x-1] == '.' {binary_value += "0";} else {binary_value += "1";}
        if space[y-1][x]   == '.' {binary_value += "0";} else {binary_value += "1";}
        if space[y-1][x+1] == '.' {binary_value += "0";} else {binary_value += "1";}

        if space[y][x-1] == '.' {binary_value += "0";} else {binary_value += "1";}
        if space[y][x]   == '.' {binary_value += "0";} else {binary_value += "1";}
        if space[y][x+1] == '.' {binary_value += "0";} else {binary_value += "1";}

        if space[y+1][x-1] == '.' {binary_value += "0";} else {binary_value += "1";}
        if space[y+1][x]   == '.' {binary_value += "0";} else {binary_value += "1";}
        if space[y+1][x+1] == '.' {binary_value += "0";} else {binary_value += "1";}

        //println!("{}", binary_value);
        let mut dec_value = usize::from_str_radix(&binary_value, 2).unwrap();
        //println!("{}", dec_value);
        new_space[y][x] = pattern[dec_value];
      }
    }

    space = new_space.clone();
 

    if new_char == '.' {new_char = '#';} else {new_char = '.';}

    let mut max_x = space[0].len()-1;
    let mut max_y = space.len()-1;

    // Deal with border
    for x in 0..max_x+1 {
      space[0][x] = new_char;
      space[max_y][x] = new_char;
    }
    for y in 0..max_y+1 {
      space[y][0] = new_char;  
      space[y][max_x] = new_char;
    }     

    // FINISHED
    //for line in space.clone() {
    //  let mut out = String::new();
    //  for c in line {
    //    out += &c.to_string();
    //  }
    //  println!("{:?}", out);
    //}

    let mut total = 0;
    for x in 1..space[0].len()-1 {
      for y in 1..space.len()-1 {
        if space[y][x] == '#' {
          total += 1;
        }
      }
    }

    //println!("TOTAL: {}", total);


  }

  let mut total = 0;
  for x in 1..space[0].len()-1 {
    for y in 1..space.len()-1 {
      if space[y][x] == '#' {
        total += 1;
      }
    }
  }

  println!("PART A: {}", total);



  
  Ok(())
}


