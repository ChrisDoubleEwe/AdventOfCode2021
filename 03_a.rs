use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("03_in.txt")?;
    let reader = BufReader::new(file);

    let mut num_digits = 0;
    let mut input_vec = Vec::new();


    for line in reader.lines() {
      let this_line = line?;
      num_digits = this_line.len();
      input_vec.push(this_line);
    }


    let mut gamma_str = String::from("");
    let mut epsilon_str = String::from("");


    for n in 0..num_digits {
      let mut num_zero = 0;
      let mut num_one = 0;
      for e in &input_vec {
        let digit = e.chars().nth(n).unwrap();
        if digit == '0' {
          num_zero += 1;
        } else {
          num_one += 1;
        }
      }


      if num_zero > num_one {
        gamma_str.push('0');
        epsilon_str.push('1');
      } else {
        gamma_str.push('1');
        epsilon_str.push('0');
      }

    }


    let gamma_dec = bin_to_dec(&gamma_str);
    let epsilon_dec = bin_to_dec(&epsilon_str);

    let power = gamma_dec * epsilon_dec;
    println!("PART A: {}", power);





    // PART B
    let mut current_vec = Vec::new();
    current_vec = input_vec.clone();

    let mut ox_gen = "";
    for n in 0..num_digits {
      let mut new_vec = Vec::new();
      let mut num_zero = 0;
      let mut num_one = 0;
      for e in &current_vec {
        let digit = e.chars().nth(n).unwrap();
        if digit == '0' {
          num_zero += 1;
        } else {
          num_one += 1;
        }
      }


      let mut looking_for = 'x';

      if num_zero > num_one {
        looking_for = '0';
      } else {
        looking_for = '1';
      }

      for e in &current_vec {
        let digit = e.chars().nth(n).unwrap();
        if digit == looking_for {
          new_vec.push(e.to_string())
        }
      }

      current_vec = new_vec.clone();
      //println!("{:?}", current_vec);
      if current_vec.len() == 1 {
        ox_gen = &current_vec[0];
        break;
      }
    }

    let ox = bin_to_dec(ox_gen);

    // scrub
    current_vec = input_vec.clone();

    let mut scrub = "";
    for n in 0..num_digits {
      let mut new_vec = Vec::new();
      let mut num_zero = 0;
      let mut num_one = 0;
      for e in &current_vec {
        let digit = e.chars().nth(n).unwrap();
        if digit == '0' {
          num_zero += 1;
        } else {
          num_one += 1;
        }
      }


      let mut looking_for = 'x';

      if num_zero <= num_one {
        looking_for = '0';
      } else {
        looking_for = '1';
      }

      for e in &current_vec {
        let digit = e.chars().nth(n).unwrap();
        if digit == looking_for {
          new_vec.push(e.to_string())
        }
      }

      current_vec = new_vec.clone();
      if current_vec.len() == 1 {
        scrub = &current_vec[0];
        break;
      }
    }

    let sc = bin_to_dec(scrub);

    let b_total = ox * sc;
    println!("PART B: {}", b_total);


    Ok(())
}

fn bin_to_dec(num: &str) -> i32 {
  let rev_num = num.chars().rev().collect::<String>();
  let mut total = 0;
  let mut mult = 1;

  for c in rev_num.chars() {
    if c == '1' {
      total += mult;
    }

    mult = mult * 2;
  }


  return total; 
}
