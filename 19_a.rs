use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;




fn main() -> io::Result<()> {
  let mut space = Vec::new();

  let mut results = Vec::new();
  results.push(0);



  let filename = "19_in.txt";
  let file = File::open(filename.clone())?;

  let reader = BufReader::new(file);

  let mut this_line = String::new();
  let mut points = Vec::new();

  for line in reader.lines() {
    this_line = line?;
    if this_line.contains(',') {
      
      let mut triple = Vec::new();

      for x in this_line.split(",") {
        triple.push(x.parse::<i32>().unwrap());
      }
      &points.push(triple.clone());
    } else if this_line.contains("---") {
      if points.len() > 0 {
        results.push(0);

        space.push(points.clone());
        for x in &points.clone() {
          &mut &points.pop();
        }
      }
    }
  }
  if points.len() > 0 {
    space.push(points);
  }

  let mut zero_points = Vec::new();
  for x in &space[0] {
    zero_points.push(x.clone());
  }
  
  println!("Warning! Takes about 5 hours to run!");
  results[0] =1;
  while results.contains(&0) {

  for scanner1 in 0..space.len() {
    for scanner2 in 0..space.len() {
      if scanner1 == scanner2 { continue; }

      if results[scanner2] == 1 {
        continue;
      }

      println!("Testing scanner {} against scanner {}", scanner1, scanner2);

      for xrot in 0..4 {
      if results[scanner2] == 1 {
        continue;
      }


        for yrot in 0..4 {
      if results[scanner2] == 1 {
        continue;
      }


          for zrot in 0..4 {
      if results[scanner2] == 1 {
        continue;
      }


            //println!("  Rotate to {},{},{}", xrot, yrot, zrot);


              for x_mod in [-1,1] {
                for y_mod in [-1,1] {
                  for z_mod in [-1,1] {

                    let mut seen = Vec::new();

                    for x in zero_points.clone() {
                      for y in &space[scanner2] {
                        let mut new_point = y.clone();
                        let mut orig_point = y.clone();

                        new_point = rotate_point(orig_point.clone(), xrot, yrot, zrot);


                        new_point[0] = new_point[0]*x_mod + x[0];
                        new_point[1] = new_point[1]*y_mod + x[1];
                        new_point[2] = new_point[2]*z_mod + x[2];
                        new_point.push(0);
                        new_point.push(0);
                        new_point.push(0);
                        new_point[3] = x_mod;
                        new_point[4] = y_mod;
                        new_point[5] = z_mod;


                        //println!("    -> {:?}", new_point.clone());
                        seen.push(new_point.clone());
                      }
                    }

                    let mut max_count = 0;
                    let mut max_vector = Vec::new();

                    for z in seen.clone() {
                      let count = &seen.iter().filter(|&n| *n == z).count().clone();
                      if count > &(max_count as usize) {
                        max_count = *count as i32;
                        max_vector = z.clone();
                      }
                    }

                    //println!("Max count: {}, s1={}, s2={}, res={}, max_vector = {:?}", max_count, scanner1, scanner2, results[scanner2], max_vector.clone());
                    if max_count >= 12 && scanner1 != scanner2 && results[scanner2] == 0 {

                      println!("Scanner {} is at {:?}, overlapping with scanner {}", scanner2, max_vector, scanner1 );
                      results[scanner2] = 1;



                      // CONVERT ALL POINTS TO ZERO-RELATIVE AND ADD AS A NEW SET
                      for y in &space[scanner2] {
  
                        let mut new_point = y.clone();
                        let mut orig_point = y.clone();
                        let mut rot_point = rotate_point(orig_point.clone(), xrot, yrot, zrot);

                        //println!(" zero-relative x: {} * {} - {}  = {}", max_vector[0], max_vector[3], rot_point[0], max_vector[0]*max_vector[3] - rot_point[0]);
                        //println!(" zero-relative y: {} * {} - {}  = {}", max_vector[1], max_vector[4], rot_point[1], max_vector[1]*max_vector[4] - rot_point[1]);
                        //println!(" zero-relative z: {} * {} - {}  = {}", max_vector[2], max_vector[5], rot_point[2], max_vector[2]*max_vector[5] - rot_point[2]);


                        new_point[0] = max_vector[0] - rot_point[0]*max_vector[3];
                        new_point[1] = max_vector[1] - rot_point[1]*max_vector[4];
                        new_point[2] = max_vector[2] - rot_point[2]*max_vector[5];


                        //println!("zero-relative point: {:?} -> {:?} ({}, {}, {}) ", orig_point, new_point, xrot, yrot, zrot);
                        zero_points.push(new_point.clone());
                      }
                      zero_points.sort_unstable();
                      zero_points.dedup();
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  zero_points.sort_unstable();
  zero_points.dedup();

  println!("PART A: {}", zero_points.len());
  Ok(())
}


fn rotate_point(vecin: Vec<i32>, xrot: i32, yrot: i32, zrot: i32) -> Vec<i32> {

  let mut c = vecin;
 
  if xrot > 0 {
    for _n in 0..xrot {
      let orig = c.clone();  
      // x=x
      c[0] = orig[0];
      // y=-z
      c[1] = orig[2]*-1;
      // z = y
      c[2] = orig[1];
    }
  }
  if yrot > 0 {
    for _n in 0..yrot {
      let orig = c.clone();
      // x=z
      c[0] = orig[2];
      // y=y
      c[1] = orig[1];
      // z = -x
      c[2] = orig[0]*-1;
    }
  }
  if zrot > 0 {
    for _n in 0..zrot {
      let orig = c.clone();
      // x=y
      c[0] = orig[1];
      // y=-x
      c[1] = orig[0]*-1;
      // z = z
      c[2] = orig[2];
    }
  }


  return c;
}

