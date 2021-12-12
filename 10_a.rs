use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let filename = "10_in.txt";
  let mut parta = 0;
  let mut partb = 0;

  let file = File::open(filename)?;
  let open = vec!['(', '[', '{', '<'];
  let close = vec![')', ']', '}', '>'];

  let mut scores = Vec::new();



  let reader = BufReader::new(file);
  let mut this_line = String::new();

  for line in reader.lines() {

    this_line = line?;
    //println!("================================");
    //println!("{}", this_line);

    let mut stack = Vec::new();

    let mut mismatch = 0;
    for c in this_line.chars() {
      //println!("   {}", c);
      if open.contains(&c) {
        stack.push(c);
        //println!("        {:?}", stack);
      } else {
        let x = stack.pop().unwrap();
        ////println!("        x: {}", x);
        //println!("        c: {}", c);

        if c == '}' && x != '{' {
          //println!("MISMATCH!!!! (curly)");
          mismatch = 1;
          parta += 1197;
          break;
        }
        if c == ']' && x != '[' {
          //println!("MISMATCH!!!! ]");
          mismatch = 1;
          parta += 57;
          break;
        }
        if c == ')' && x != '(' {
          //println!("MISMATCH!!!! )");
          mismatch = 1;
          parta += 3;
          break;
        }
        if c == '>' && x != '<' {
          //println!("MISMATCH!!!! >");
          mismatch = 1;
          parta += 25137;
          break;
        }

      }

    } 
    if mismatch == 0 {
      //println!("INCOMPLETE!");
      let mut score: u64  = 0;

      while stack.len() > 0 {
        let x = stack.pop().unwrap();
        //println!("  {}", x);
        if x == '(' { score = score *5 ; score += 1;}
        if x == '[' { score = score *5 ; score += 2;}
        if x == '{' { score = score *5 ; score += 3;}
        if x == '<' { score = score *5 ; score += 4;}
      }
      //println!("   SCORE: {}", score);
      scores.push(score);
    }
  }
  scores.sort();
  let length = scores.len();
  let idx = length/2;
  partb = scores[idx];
  println!("PART A: {}", parta);
  println!("PART B: {}", partb);
  Ok(())
}

