use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;


fn main() -> io::Result<()> {
  let filename = "14_in.txt";
  let file = File::open(filename.clone())?;

  let reader = BufReader::new(file);


  let mut pairs= Vec::new();
  let mut p1 = String::new();
  let mut p2 = String::new();
  let mut p3 = String::new();

  let mut pair = (p1, p2, p3);

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
      let from_line = this_line.clone();
      let from = from_line.split(" -> ").nth(0).unwrap();
      let from1 = from.clone().chars().nth(0).unwrap();
      let from2 = from.clone().chars().nth(1).unwrap();

      let to_line = this_line.clone();
      let to = to_line.split(" -> ").nth(1).unwrap().chars().nth(0).unwrap();
      let to1 = from1.clone().to_string() + &to.to_string();
      let to2 = to.clone().to_string() + &from2.to_string();


      let mut pair_new = (String::new(), String::new(), String::new());
      pair_new.0 = from.to_string().clone();
      pair_new.1 = to1.clone();
      pair_new.2 = to2.clone();
      pairs.push(pair_new);
    }
  }

  //println!("Template: {}", initial);
  //println!("{:?}", pairs);

  //for n in 1..40+1 {
  //  let mut new_initial = String::new();
  //  for i in 0..initial.len() {
  //    if i == initial.len()-1 {
  //      new_initial += &initial.chars().nth(i).unwrap().to_string();
  //    } else {
  //      //println!("{} / {}", i, initial.len());
  //      let from1 = initial.chars().nth(i).unwrap();
  //      let from2 = initial.chars().nth(i+1).unwrap();
  //      for pair in pairs.clone() {
  //        if pair.0 == from1 && pair.1 == from2 {
  //          new_initial += &pair.0.to_string();
  //          new_initial += &pair.2.to_string();
  //        }
  //      }
  //    }
  //  }
  //  println!("After step {}", n);
  //  initial = new_initial.clone()
  //}

  
  let mut initial_vec = Vec::new();
 
  for i in 0..initial.len()-1 {
    let mut new_p = String::new();
    new_p += &initial.chars().nth(i).unwrap().to_string();
    new_p += &initial.chars().nth(i+1).unwrap().to_string();
    initial_vec.push(new_p);
  }
  //println!("{:?}", initial_vec);

  let mut the_map = HashMap::new();
  for pair in initial_vec {
    let count = the_map.entry(pair).or_insert(0);
    *count += 1;
  }

  for step in 1..40+1 {
    let mut new_the_map = HashMap::new();
    for (key, value) in the_map.into_iter() {
      for tuple in pairs.clone() {
        if tuple.0 == key.clone() {
          let count1 = new_the_map.entry(tuple.1.clone()).or_insert(0);
          *count1 += value;
          let count2 = new_the_map.entry(tuple.2.clone()).or_insert(0);
          *count2 += value;

          //let count = new_the_map.entry(key.clone()).or_insert(0);
        }
      }
    }
    the_map = new_the_map.clone();
  }

  //println!("----> {:?}", the_map);
  let mut results_map = HashMap::new();
  for (key, value) in the_map.into_iter() {
    let first = key.chars().nth(0).unwrap();
    let count1 = results_map.entry(first.clone()).or_insert(0);
    *count1 += value;
    let second = key.chars().nth(1).unwrap();
    let count2 = results_map.entry(second.clone()).or_insert(0);
    *count2 += value;
  }

  //println!("{:?}", results_map);
  let start = initial.clone().chars().nth(0).unwrap();
  //println!("START: {}", start);
  let end = initial.clone().chars().nth(initial.len()-1).unwrap();
  //println!("END: {}", end);
  let count_start = results_map.entry(start.clone()).or_insert(0);
  *count_start += 1;
  let count_end = results_map.entry(end.clone()).or_insert(0);
  *count_end += 1;


  //println!("{:?}", results_map);

  let mut final_results_map = HashMap::new();
  final_results_map = results_map.clone();

  for (key, value) in results_map.into_iter() {
    let count = final_results_map.entry(key.clone()).or_insert(0);

    *count = value/2;
  }

  //println!("{:?}", final_results_map.clone());

  let mut max_res = 0;
  let mut min_res: u64= 999999999999999999;

  for (key, value) in final_results_map.into_iter() {
    if value > max_res { max_res = value;}
    if value < min_res { min_res = value;}
  }

  //println!("MAX: {}", max_res);
  //println!("MIN: {}", min_res);

  let partb = max_res - min_res;

  println!("PART B: {}", partb);




  Ok(())
}

