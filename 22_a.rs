use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;




fn main() -> io::Result<()> {
  let mut core = Vec::new();
  let mut core_x = Vec::new();
  let mut core_y = Vec::new();


  let cube_size = 100;

  for x in 0..cube_size+1 {
    core_x.push(0);
  }

  for y in 0..cube_size+1 {
    core_y.push(core_x.clone());
  }

  for z in 0..cube_size+1 {
    core.push(core_y.clone());
  }

  let filename = "22_in.txt";
  let file = File::open(filename.clone())?;

  let reader = BufReader::new(file);

  let mut this_line = String::new();

  for line in reader.lines() {
    this_line = line?;
    if this_line.contains(',') {
      let mut on_off = this_line.split(" ").nth(0).unwrap();

      let mut c_vec = Vec::new();

      let mut skip = 0;

      for c in this_line.split(" ").nth(1).unwrap().split(",") {
        let mut pair = Vec::new();

        let mut coords = c.split("=").nth(1).unwrap();
        let mut n = coords.split('.');

        let mut coord1 = n.clone().nth(0).unwrap().parse::<i32>().unwrap();
        let mut coord2 = n.clone().nth(2).unwrap().parse::<i32>().unwrap();
        coord1 += cube_size/2;
        coord2 += cube_size/2;

        if coord2 < coord1 {
          let mut tmp = coord1;
          coord1 = coord2;
          coord2 = tmp;
        }
        if coord1 < 0 && coord2 < 0 {skip = 1;}
        if coord1 > cube_size && coord2 > cube_size {skip = 1;}

        if coord1 < 0 { coord1 = 0; }
        if coord2 < 0 { coord2 = 0; }
        if coord1 > cube_size { coord1 = cube_size; }
        if coord2 > cube_size { coord2 = cube_size; }

        pair.push(coord1);
        pair.push(coord2);
        c_vec.push(pair.clone());
      }

      if skip == 1 { 
        println!("skip!");
        continue; 
      }

      println!("{}   {:?}", on_off, c_vec);

      for x in c_vec[0][0]..c_vec[0][1]+1 {
        for y in c_vec[1][0]..c_vec[1][1]+1 {
          for z in c_vec[2][0]..c_vec[2][1]+1 {
            if on_off == "on" {
              core[x as usize][y as usize][z as usize] = 1;
              //println!("Setting {},{},{} to 1", x, y, z);
            } else {
              core[x as usize][y as usize][z as usize] = 0;
              //println!("Setting {},{},{} to 0", x, y, z);
            }
          }
        }
      }
    }
  }

  let mut cubes_on = 0;
  for x in 0..cube_size+1 {
    for y in 0..cube_size+1 {
      for z in 0..cube_size+1 {
        if core[x as usize][y as usize][z as usize] == 1 {cubes_on += 1;}
      }
    }
  }

  println!("Cubes on: {}", cubes_on);

  Ok(())
}


