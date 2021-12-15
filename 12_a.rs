use std::fs::File;
use std::io::{self, prelude::*, BufReader};

static mut num_paths: i32 = 0;

fn main() -> io::Result<()> {
  let filename = "12_in.txt";
  let file = File::open(filename.clone())?;

  let reader = BufReader::new(file);
  let mut connections = Vec::new();

  let this_line = String::new();


  for line in reader.lines() {
    let this_line = line?;
    let from = this_line.split("-").nth(0).unwrap();
    let to = this_line.split("-").nth(1).unwrap();

    let c = (from.to_string(), to.to_string());
    connections.push(c);

    let rev_c = (to.to_string(), from.to_string());
    connections.push(rev_c);

  }

  //println!("{:?}",connections);

  let mut this_path = Vec::new();
  this_path.push("start".to_string());
  path(&connections, "start".to_string(), &mut this_path);

  unsafe{println!("PART A: {}", num_paths);}
  Ok(())
}

fn path(connections: &Vec<(String,String)>, node: String, p: &mut Vec<String>) {
  for pair in connections {
    let mut new_p = p.clone();
    //println!("{:?}", pair);
    if pair.0 == node {
      //println!("Found match!");
 
      
      // Only visit lower case caves once
      let mut goin = 1;

      if pair.1.to_lowercase() == pair.1 {
        if new_p.contains(&pair.1) {
          goin = 0;
        }
      }
           
        
      if goin == 1 {
        new_p.push(pair.1.clone());
        if pair.1 == "end" {
          //println!("END: {:?}", new_p);
          unsafe {num_paths += 1;}
        } else {
          //println!("CALLING PATH: {:?}", new_p);
          path(&connections, pair.1.to_string(), &mut new_p);
        }
      }


    }
  }
  return;    
}
  

