use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;


fn main() -> io::Result<()> {
  let filename = "18_in.txt";
  let file = File::open(filename.clone())?;

  let reader = BufReader::new(file);

  let mut this_line = String::new();

  let mut lines = Vec::new();

  for line in reader.lines() {
    this_line = line?;
    lines.push(this_line.clone());
  }

  let result = doit(lines.clone());


  let mut max_result = 0;

  for x in lines.clone() {
    for y in lines.clone() {
      let mut pair = Vec::new();
      pair.push(x.clone());
      pair.push(y.clone());
      let result = doit(pair.clone());
      if result > max_result {
        max_result = result;
      }
    }
  }

  println!("PART B: {}", max_result);

  Ok(())
}


fn doit(lines: Vec<String>) -> (i32) {
  let mut line1 = Vec::new();


  for i in 0..lines.len() {
    let mut line2 = Vec::new();
    let mut number = Vec::new();


    if i == 0 {
      for c in lines[i].chars() { 
        if c == '[' {line1.push(-66);}
        else if c == ']' {line1.push(-99);}
        else if c == ',' {line1.push(-1);}
        else {line1.push(c.to_string().parse::<i32>().unwrap());}
      }
    } else {
      for c in lines[i].chars() {
        if c == '[' {line2.push(-66);}
        else if c == ']' {line2.push(-99);}
        else if c == ',' {line2.push(-1);}
        else {line2.push(c.to_string().parse::<i32>().unwrap());}
      }
      number.push(-66);
      number.extend(line1.clone());
      number.push(-1);
      number.extend(line2.clone());
      number.push(-99);


      // REDUCE LINE
      let mut exploded = 1;
      let mut split = 1;

      while exploded == 1 || split == 1 {
        exploded = 0;
        split = 0;
        // 1. EXPLODE
        let mut depth = 0;
        for idx in 0..number.len() {
          if number[idx] == -66 { 
            depth += 1;
            continue;
          }
          if number[idx] == -99 {
            depth -= 1;
            continue;
          }
          if idx >=1 && idx <= (number.len()-3) && depth >= 5 && number[idx] >= 0 && number[idx-1] == -66 && number[idx+1] == -1 && number[idx+2] >= 0 && number[idx+3] == -99 {

            // ADD LEFT
            for left in 1..idx {
              if number[idx-left] >= 0 {
                number[idx-left] += number[idx];
                break;
              }
            }
            // ADD RIGHT 
            for right in 3..number.len()-idx {
              if number[idx+right] >= 0 {
                //println!("Adding {} to {}", number[idx+2], number[idx+right]);
                number[idx+right] += number[idx+2];
                break;
              }
            }
            // REPLACE PAIR WITH ZERO
            //println!("Replace pair with zero at {}:", idx);
            //my_println(number.clone());
            //my_printspace(number.clone(), idx.clone());

            number[idx] = 0; // LEFT VALUE
            //my_println(number.clone());
            number.remove(idx+1); // COMMA
            //my_println(number.clone());
            number.remove(idx+1); // RIGHT VALUE
            //my_println(number.clone());
            number.remove(idx+1); // CLOSE BRACE
            //my_println(number.clone());
            number.remove(idx-1); // OPENE BRACE
            exploded = 1;
            break;
          }
        }
        // 2. SPLIT
        if exploded == 0 {

          for idx in 0..number.len() {
            if number[idx] >= 10 {


              let mut this_num = (number[idx] as f32 / 2.0);
              let mut this_num_down = this_num.floor() as i32;
              let mut this_num_up = this_num.ceil() as i32;

              number.remove(idx); // orig number
              number.insert(idx, -99); // ]
              number.insert(idx, this_num_up); // rounded up
              number.insert(idx, -1); // ,
              number.insert(idx, this_num_down); // rounded up
              number.insert(idx, -66); // ]

              split = 1;

              break;
            }
          }
        }
      }
      line1 = number.clone();
    }
  }


  
  let mut line3 = line1.clone();
  //let mut line3 = Vec::new();
  //let mut string3 = "[9,1]";
  //let mut string3 = "[[9,1],[1,9]]";
  //let mut string3 = "[[1,2],[[3,4],5]]";
  //let mut string3 = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
  //let mut string3 = "[[[[1,1],[2,2]],[3,3]],[4,4]]";
  //let mut string3 = "[[[[3,0],[5,3]],[4,4]],[5,5]]";
  //let mut string3 = "[[[[5,0],[7,4]],[5,5]],[6,6]]";
  //let mut string3 = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";

  //for c in string3.chars() {
  //  if c == '[' {line3.push(-66);}
  //  else if c == ']' {line3.push(-99);}
  //  else if c == ',' {line3.push(-1);}
  //  else {line3.push(c.to_string().parse::<i32>().unwrap());}
  //}
  //println!("{:?}", line3); 
 
  while line3.len() > 1 {
    for idx in 0..line3.len()-3 {
      if line3[idx]==-66 && line3[idx+1] >= 0 && line3[idx+2] == -1 && line3[idx+3] >= 0 && line3[idx+4] == -99 {
        line3[idx] = 3*line3[idx+1] + 2*line3[idx+3];
        line3.remove(idx+1); // VAL
        line3.remove(idx+1); // COMMA
        line3.remove(idx+1); // VAL
        line3.remove(idx+1); // CLOSE
        break;
      }
    }
  }

  return line3[0] as i32;
}

fn my_println(c: Vec<i32>) {
  let mut out = String::new();

  for i in c {
    if i == -66 {out += "["};
    if i == -1 {out += ","};
    if i == -99 {out += "]"};
    if i >= 0 {out += &i.to_string();}
  }

  println!("{}", out);
}

fn my_printspace(c: Vec<i32>, idx: usize) {
  let mut out = String::new();
  for i in 0..idx {
    if c[i] == -66 {out += "["};
    if c[i] == -1 {out += ","};
    if c[i] == -99 {out += "]"};
    if c[i] >= 0 {out += &c[i].to_string();}
  }

  out += "^";

  println!("{}", out);
}

