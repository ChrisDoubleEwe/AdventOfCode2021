use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;


fn main() -> io::Result<()> {
  let mut result_string = String::new();


  let mut max_num = 1000000;

  let mut regs = [0, 0, 0, 0];
  // add x 10
  let mut numbers1 = [10, 14, 14, -13, 10, -13, -7, 11, 10, 13, -4, -9, -13, -9];
  // div z 1 or 26
  let mut numbers2 = [1, 1, 1, 26, 1, 26, 26, 1, 1, 1, 26, 26, 26, 26];

  // add y num
  let mut numbers3 = [2, 13, 13, 9, 15, 3, 6, 5, 16, 1, 6, 3, 7, 9];



  let mut z_results = Vec::new();
  let mut z_empty: Vec::<i32> = Vec::new();
  for i in 0..max_num+1 {
    z_empty.push(0);
  }
  for i in 0..15 {
    z_results.push(z_empty.clone());
  }
  z_results[14][0] = 1;

  println!("Big array done");



  for stage in (0..14).rev() { 
    println!("stage: {}" , stage);


  for input in 1..10 {
   for input_z in 0..max_num {

    let mut regs = [0, 0, 0, 0];
    regs[3] = input_z;

    let mut add_x = numbers1[stage];
    let mut div_z = numbers2[stage];
    let mut add_y = numbers3[stage];

    //println!("{} {} {}", add_x, div_z, add_y);

    // inp w
    regs[0] = input;

    // mul x 0
    // add x z
    regs[1] = regs[3];

    // mod x 26
    let mut sum = regs[1] % 26 ;
    regs[1] = sum.clone();

    // div z 1			<-- sometimes this number ; 1 or 26
    let mut sum = regs[3] / div_z ;
    regs[3] = sum.clone();

    // add x 10    <-- number here
    regs[1] += add_x;

    // eql x w
    // eql x 0
    if regs[1] != regs[0] {regs[1]=1;} else {regs[1]=0;}

    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    regs[2] = ( regs[1] * 25 ) + 1;
 
    // mul z y
    regs[3] = regs[3] * regs[2];

    // mul y 0
    // add y w
    // add y 2    <- number here
    regs[2] = regs[0] + add_y;

    // mul y x
    regs[2] = regs[2] * regs[1];

    // add z y
    regs[3] += regs[2];

    if regs[3].clone() <= max_num {
    //println!("  checking stage+1: {} z: {} table: {}", stage+1, regs[3], z_results[stage+1][regs[3].clone() as usize]);

    if z_results[stage+1][regs[3].clone() as usize] == 1 {
      //println!("Stage {} : input {} with a previous z of {} produces a new z of {}", stage, input, input_z, regs[3]);
      z_results[stage][input_z as usize] = 1;
    }
    }
   }
  }

  }


  // OK NOW TRY AND WORK OUT THE ANSWER
  let mut input_z = 0;
  for digit in 0..14 {
    println!("Doing digit: {}", digit);

    //println!("    Looking for the following: ");
    //for i in 0..max_num+1 {
    //  if z_results[digit+1][i as usize] == 1 {
    //    println!("          {}", i);
    //  }
   // }

    
    for value in 1..10 {
      //println!("  Trying value: {}", value);

      let mut regs = [0, 0, 0, 0];
      regs[3] = input_z;

      let mut add_x = numbers1[digit];
      let mut div_z = numbers2[digit];
      let mut add_y = numbers3[digit];


      // inp w
      regs[0] = value.clone();

      // mul x 0
      // add x z
      regs[1] = regs[3];

      // mod x 26
      let mut sum = regs[1] % 26 ;
      regs[1] = sum.clone();

      // div z 1                  <-- sometimes this number ; 1 or 26
      let mut sum = regs[3] / div_z ;
      regs[3] = sum.clone();

      // add x 10    <-- number here
      regs[1] += add_x;

      // eql x w
      // eql x 0
      if regs[1] != regs[0] {regs[1]=1;} else {regs[1]=0;}

      // mul y 0
      // add y 25
      // mul y x
      // add y 1
      regs[2] = ( regs[1] * 25 ) + 1;

      // mul z y
      regs[3] = regs[3] * regs[2];

      // mul y 0
      // add y w
      // add y 2    <- number here
      regs[2] = regs[0] + add_y;

      // mul y x
      regs[2] = regs[2] * regs[1];

      // add z y
      regs[3] += regs[2];

      //println!("     result z = {}", regs[3]);

      if z_results[digit+1][regs[3].clone() as usize] == 1 {
        //println!("GOTCHA - digit {} is {}", digit, value);
        result_string += &value.to_string();
        input_z = regs[3].clone();
        break;
      }

   
    }
  }
  println!("RESULT: {}", result_string);
  Ok(())
}

