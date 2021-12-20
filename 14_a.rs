use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let filename = "14_in.txt";
  let file = File::open(filename.clone())?;

  let reader = BufReader::new(file);


  let mut pairs= Vec::new();
  let mut pair = ('z', 'z', 'z');

  let mut this_line = String::new();
  
  let mut initial = String::new();
  let mut first = 1;

  for line in reader.lines() {
    this_line = line?;
    if first == 1 {
      initial = this_line.clone();
      first = 0;
    }
    if this_line.contains(" -> ") {
      let from = this_line.split(" -> ").nth(0).unwrap();
      let from1 = from.chars().nth(0).unwrap();
      let from2 = from.chars().nth(1).unwrap();

      let to = this_line.split(" -> ").nth(1).unwrap().chars().nth(0).unwrap();

      pair.0 = from1;
      pair.1 = from2;
      pair.2 = to;
      pairs.push(pair);
    }
  }

  //println!("Template: {}", initial);
  //println!("{:?}", pairs);

  for n in 1..10+1 {
    let mut new_initial = String::new();
    for i in 0..initial.len() {
      if i == initial.len()-1 {
        new_initial += &initial.chars().nth(i).unwrap().to_string();
      } else {
        //println!("{} / {}", i, initial.len());
        let from1 = initial.chars().nth(i).unwrap();
        let from2 = initial.chars().nth(i+1).unwrap();
        for pair in pairs.clone() {
          if pair.0 == from1 && pair.1 == from2 {
            new_initial += &pair.0.to_string();
            new_initial += &pair.2.to_string();
          }
        }
      }
    }
    //println!("After step {}", n);
    initial = new_initial.clone()
  }

  
  let mut chars= Vec::new();
  let mut count= Vec::new();


  for c in initial.chars() {
    if chars.contains(&c) {
      // skip
      let mut dummy = 0;
    } else {
      chars.push(c);
    }
  }

  for c in chars.clone() {
    count.push(0);
  }

  for c in initial.chars() {
    let index = chars.iter().position(|&r| r == c).unwrap();
    count[index] += 1;
  }

  let mut max_res = 0;
  let mut min_res: u64= 999999999999999999;
  for c in count {
   if c > max_res { max_res = c;}
   if c < min_res { min_res = c;}
  }

  let parta = max_res - min_res;


    


  println!("PART A: {}", parta);

  Ok(())
}

