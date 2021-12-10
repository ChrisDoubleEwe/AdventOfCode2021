use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let file = File::open("08_in.txt")?;
  let reader = BufReader::new(file);

  let mut this_line = String::new();
  let mut parta = 0;
  let mut partb = 0;

  for line in reader.lines() {
    //println!("------NEW LINE---------------");
    this_line = line?;
    //println!("{}", this_line);

    let output_digits = this_line.split("|").nth(1).unwrap().split(" ");

    let mut output_string = String::new();
    output_string += " ";
    //let mut o_s = this_line.split("|").nth(1).unwrap().to_string();

    for s in output_digits.clone() {
      let mut l: Vec<char> = s.chars().collect();
      l.sort_unstable();
      let j: String = l.into_iter().collect();

      output_string += &j;
      output_string += " ";
    }

    output_string = output_string.replace(" ", "  ");


    let input_digits = this_line.split("|").nth(0).unwrap().split(" ");


    for d in output_digits {
      if d.len() == 2 || d.len() == 4 || d.len() == 3 || d.len() == 7 {
        parta += 1;
        //println!("{} ; {}", d, d.len());
      }
    }

    let mut values = vec!["".to_string(); 10];
    let mut val = String::new();


    for _i in 0..10 {
      //println!("{:?}", values);
      for v in input_digits.clone() {
        let mut va: Vec<char> = v.chars().collect();
        va.sort_unstable();
        let vals: String = va.into_iter().collect();
        val = vals.to_string();

        //println!("{}", val);
        // 1
        if val.len() == 2 {
          values[1] = val.clone();
          //println!("{} is 1", val.clone());
        }
        // 7
        if val.len() == 3 {
          values[7] = val.clone();
          //println!("{} is 7", val.clone());
        }
        // 4
        if val.len() == 4 {
          values[4] = val.clone();
          //println!("{} is 4", val.clone());
        }
        // 8
        if val.len() == 7 {
          values[8] = val.clone();
          //println!("{} is 8", val.clone());
        }
        // 3
        if val.len() == 5 && values[1].len() == 2 {
          let mut flag = 1;
          for x in values[1].chars() {
            if val.contains(x) {
              //
            } else {
              flag = 0;
            }
          }
          if flag == 1 {
            values[3] = val.clone();
          //println!("{} is 3", val.clone());
          }
        }
        // 9
        if val.len() == 6 && values[4].len() > 1 && values[3].len() > 1 {
          let mut flag_four = 1;
          let mut flag_three = 1;

          for x in values[4].chars() {
            if val.contains(x) {
              //
            } else {
              flag_four = 0;
            }
          }
          for x in values[3].chars() {
            if val.contains(x) {
              //
            } else {
              flag_three = 0;
            }
          }

          if flag_four == 1 && flag_three == 1{
            values[9] = val.clone();
            //println!("{} is 9", val.clone());
          }
        }
        // 2
        if val.len() == 5 && values[3].len() > 1 && values[1].len() > 1 && values[4].len() > 1 {
          let mut flag_one = 0;
          let mut missing_one = 'x';
          let mut flag_three = 0;
          let mut missing_three = 'y';
          let mut flag_four = 0;

          for x in values[4].chars() {
            if val.contains(x) {
              flag_four += 1;
            }
          }


          for x in values[1].chars() {
            if val.contains(x) {
              flag_one += 1;
            } else {
              missing_one = x.clone();
            }
          }
          for x in values[3].chars() {
            if val.contains(x) {
              flag_three += 1;
            } else {
              missing_three = x.clone();
            }
          }

          if flag_four == values[4].len() - 2 && flag_one == values[1].len() -1  && flag_three == values[3].len() -1  && missing_one == missing_three{
            values[2] = val.clone();
            //println!("{} is 2", val.clone());
          }
        }

        // 5
        if val.len() == 5 && values[4].len() > 1 && values[1].len() > 1 {
          let mut flag_one = 0;
          let mut missing_one = 'x';
          let mut flag_four = 0;
          let mut missing_four = 'y';


          for x in values[1].chars() {
            if val.contains(x) {
              flag_one += 1;
            } else {
              missing_one = x.clone();
            }
          }
          for x in values[4].chars() {
            if val.contains(x) {
              flag_four += 1;
            } else {
              missing_four = x.clone();
            }
          }

          if flag_one == values[1].len() -1  && flag_four == values[4].len() -1  && missing_one == missing_four{
            values[5] = val.clone();
            //println!("{} is 5", val.clone());
          }
        }
        // 6
        if val.len() == 6 && values[5].len() > 1 && values[7].len() > 1 {
          let mut flag_five = 0;
          let mut flag_seven = 0;


          for x in values[5].chars() {
            if val.contains(x) {
              flag_five += 1;
            }
          }
          for x in values[7].chars() {
            if val.contains(x) {
              flag_seven += 1;
            }
          }

          if flag_five == values[5].len() && flag_seven == values[7].len() -1 {
            values[6] = val.clone();
            //println!("{} is 6", val.clone());
          }
        }
        // 0
        if val.len() == 6 && values[2].len() > 1 && values[5].len() > 1 {
          let mut flag_two = 0;
          let mut missing_two = 'x';
          let mut flag_five = 0;
          let mut missing_five = 'y';


          for x in values[2].chars() {
            if val.contains(x) {
              flag_two += 1;
            } else {
              missing_two = x.clone();
            }
          }
          for x in values[5].chars() {
            if val.contains(x) {
              flag_five += 1;
            } else {
              missing_five = x.clone();
            }
          }

          if flag_two == values[2].len() -1  && flag_five == values[5].len() -1  && missing_two == missing_five{
            values[0] = val.clone();
            //println!("{} is 0", val.clone());

          }
        }

      }
    }
    let mut space_values = vec!["".to_string(); 10];

    let mut idx = 0;
    for x in values.clone() {
      let mut space_val = String::new();
      space_val += " ";
      space_val += &x.to_string();
      space_val += " ";
      space_values[idx] = space_val.clone();
      idx += 1;
    }
    //println!("space_values: {:?}", space_values);

    //println!("BEFORE: {:?}", output_string);

    //let mut result = str::replace(&output_string, values[0], "0");
    //result = str::replace(&result, values[1], "1");
    let mut result = output_string.replace(&space_values[0].to_string(), "0");
    result = result.replace(&space_values[1].to_string(), " 1 ");
    result = result.replace(&space_values[2].to_string(), " 2 ");
    result = result.replace(&space_values[3].to_string(), " 3 ");
    result = result.replace(&space_values[4].to_string(), " 4 ");
    result = result.replace(&space_values[5].to_string(), " 5 ");
    result = result.replace(&space_values[6].to_string(), " 6 ");
    result = result.replace(&space_values[7].to_string(), " 7 ");
    result = result.replace(&space_values[8].to_string(), " 8 ");
    result = result.replace(&space_values[9].to_string(), " 9 ");
    result = result.replace(&space_values[0].to_string(), " 0 ");
    result = result.replace(" ", "");

    //println!("AFTER: {:?}", result);
    let res_num = result.parse::<i32>().unwrap();
    partb += res_num;
    //println!("{}", res_num);




  }

  println!("PART A: {}", parta);
  println!("PART B: {}", partb);

  Ok(())
}

