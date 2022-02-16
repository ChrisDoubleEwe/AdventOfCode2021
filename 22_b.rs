use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;




fn main() -> io::Result<()> {
  let mut xs_from = Vec::new();
  let mut xs_to = Vec::new();

  let mut ys_from = Vec::new();
  let mut ys_to = Vec::new();

  let mut zs_from = Vec::new();
  let mut zs_to = Vec::new();


  let filename = "22_in.txt";
  let file = File::open(filename.clone())?;
  let file1 = File::open(filename.clone())?;

  let reader = BufReader::new(file);

  let mut this_line = String::new();

  let reader1 = BufReader::new(file1);
  let mut max_num = 0;

  for line in reader1.lines() {
    this_line = line?;
    if this_line.contains(',') {
      let x_range = this_line.split(" ").nth(1).unwrap().split(",").nth(0).unwrap();
      let mut coords = x_range.split("=").nth(1).unwrap();
      let mut n = coords.split('.');
      let mut coord1 = n.clone().nth(0).unwrap().parse::<i32>().unwrap();
      let mut coord2 = n.clone().nth(2).unwrap().parse::<i32>().unwrap();
      xs_from.push(coord1);
      xs_to.push(coord1-1);

      xs_to.push(coord2);
      xs_from.push(coord2+1);


      let y_range = this_line.split(" ").nth(1).unwrap().split(",").nth(1).unwrap();
      coords = y_range.split("=").nth(1).unwrap();
      n = coords.split('.');
      coord1 = n.clone().nth(0).unwrap().parse::<i32>().unwrap();
      coord2 = n.clone().nth(2).unwrap().parse::<i32>().unwrap();
      ys_from.push(coord1);
      ys_to.push(coord1-1);

      ys_to.push(coord2);
      ys_from.push(coord2+1);

      let z_range = this_line.split(" ").nth(1).unwrap().split(",").nth(2).unwrap();
      coords = z_range.split("=").nth(1).unwrap();
      n = coords.split('.');
      coord1 = n.clone().nth(0).unwrap().parse::<i32>().unwrap();
      coord2 = n.clone().nth(2).unwrap().parse::<i32>().unwrap();
      zs_from.push(coord1);
      zs_to.push(coord1-1);

      zs_to.push(coord2);
      zs_from.push(coord2+1);


      

    }
  }

  xs_to.sort_unstable();
  xs_to.dedup();
  xs_from.sort_unstable();
  xs_from.dedup();
  println!("xs_from: {:?}", xs_from);
  println!("xs_to: {:?}", xs_to);

  xs_from.pop();
  xs_to.remove(0);


  ys_to.sort_unstable();
  ys_to.dedup();
  ys_from.sort_unstable();
  ys_from.dedup();
  ys_from.pop();
  ys_to.remove(0);




  zs_to.sort_unstable();
  zs_to.dedup();
  zs_from.sort_unstable();
  zs_from.dedup();
  zs_from.pop();
  zs_to.remove(0);







  println!("X from: {:?}", xs_from.clone());
  println!("X   to: {:?}", xs_to.clone());

  println!("Y from: {:?}", ys_from.clone());
  println!("Y   to: {:?}", ys_to.clone());

  println!("Z from: {:?}", zs_from.clone());
  println!("Z   to: {:?}", zs_to.clone());


  // Create core map
  let mut core = Vec::new();
  let mut core_x = Vec::new();
  let mut core_y = Vec::new();

  let mut max_core = xs_from.len();
  if xs_to.len() > max_core { max_core = xs_to.len(); }
  if ys_to.len() > max_core { max_core = ys_to.len(); }
  if zs_to.len() > max_core { max_core = zs_to.len(); }
  if zs_from.len() > max_core { max_core = zs_from.len(); }
  if ys_from.len() > max_core { max_core = ys_from.len(); }







  for x in 0..max_core+1 {
    core_x.push(0);
  }

  for y in 0..max_core+1 {
    core_y.push(core_x.clone());
  }

  for z in 0..max_core+1 {
    core.push(core_y.clone());
  }



  for line in reader.lines() {
    this_line = line?;
    if this_line.contains(',') {
      let mut on_off = this_line.split(" ").nth(0).unwrap();

      let x_range = this_line.split(" ").nth(1).unwrap().split(",").nth(0).unwrap();
      let mut coords = x_range.split("=").nth(1).unwrap();
      let mut n = coords.split('.');
      let mut x_from = n.clone().nth(0).unwrap().parse::<i32>().unwrap();
      let mut x_to = n.clone().nth(2).unwrap().parse::<i32>().unwrap();

      let y_range = this_line.split(" ").nth(1).unwrap().split(",").nth(1).unwrap();
      coords = y_range.split("=").nth(1).unwrap();
      n = coords.split('.');
      let mut y_from = n.clone().nth(0).unwrap().parse::<i32>().unwrap();
      let mut y_to = n.clone().nth(2).unwrap().parse::<i32>().unwrap();

      let z_range = this_line.split(" ").nth(1).unwrap().split(",").nth(2).unwrap();
      coords = z_range.split("=").nth(1).unwrap();
      n = coords.split('.');
      let mut z_from = n.clone().nth(0).unwrap().parse::<i32>().unwrap();
      let mut z_to = n.clone().nth(2).unwrap().parse::<i32>().unwrap();

      println!("{}..{}, {}..{}, {}..{}", x_from, x_to, y_from, y_to, z_from, z_to);
      for x in 0..xs_from.clone().len() {
        for y in 0..ys_from.clone().len() {
          for z in 0..zs_from.clone().len() {
            //println!("trying {},{},{}", xs_from[x], ys_from[y], zs_from[z]);
            if xs_from[x] >= x_from && xs_to[x] <= x_to && ys_from[y] >= y_from && ys_to[y] <= y_to && zs_from[z] >= z_from && zs_to[z] <= z_to {
              if on_off == "on" {
                //println!("turning on {},{},{}", xs_from[x], ys_from[y], zs_from[z]);
                core[x][y][z] = 1;
              } else {
                //println!("turning off {},{},{}", xs_from[x], ys_from[y], zs_from[z]);
                core[x][y][z] = 0;
              }
            }
          }
        }
      }
    }
  }

  let mut total_cubes: i64 = 0;


  println!("Checking x: {}..{}", 0, xs_from.clone().len());
  for x in 0..xs_from.clone().len() {
    for y in 0..ys_from.clone().len() {
      for z in 0..zs_from.clone().len() {
        println!(" checking {}, {}, {}", x, y, z);
        if core[x][y][z] == 1 {
          println!("This cube is on: {}..{}, {}..{}, {}..{}", xs_to[x], xs_from[x], ys_to[y], ys_from[y], zs_to[z], zs_from[z]);
          let add_cubes: i64 = ((xs_to[x] as i64 - xs_from[x] as i64) +1) * ((ys_to[y] as i64 - ys_from[y] as i64)+1) * ((zs_to[z] as i64 - zs_from[z] as i64)+1);
          println!("  adding {} cubes", add_cubes);
          total_cubes += add_cubes as i64;  
        }
      }
    }
  }

  println!("TOTAL CUBES: {}", total_cubes);

  Ok(())
}


