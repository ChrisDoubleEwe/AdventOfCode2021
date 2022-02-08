use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;




fn main() -> io::Result<()> {
  let mut points = Vec::new();



  let filename = "19_scanners.txt";
  let file = File::open(filename.clone())?;

  let reader = BufReader::new(file);

  let mut this_line = String::new();

  for line in reader.lines() {
    this_line = line?;

    if this_line.contains(',') {
      let mut triple = Vec::new();
      for x in this_line.split(",") {
        triple.push(x.parse::<i32>().unwrap());
      }
      &points.push(triple.clone());
    } 
  }


  let mut max_dist = 0;

  for from in points.clone() {
    for to in points.clone() {
      let distx = ( from[0] - to[0] );
      let disty = ( from[1] - to[1] );
      let distz = ( from[2] - to[2] );

      let dist = distx.abs() + disty.abs() + distz.abs();
      if dist > max_dist {
        max_dist = dist;
      }

      //println!("From: {:?} ; To: {:?} = DIST: {}", from.clone(), to.clone(), dist);
    }
  }

  println!("PART B: {}" , max_dist);

  Ok(())
}
