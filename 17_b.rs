use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;


fn main() -> io::Result<()> {

  let mut x: i32 = 0;
  let mut y: i32 = 0;

  //let target_min_x: i32 = 20;
  //let target_max_x: i32 = 30;
  //let target_min_y: i32 = -10;
  //let target_max_y: i32 = -5;
  let target_min_x: i32 = 269;
  let target_max_x: i32 = 292;
  let target_min_y: i32 = -68;
  let target_max_y: i32 = -44;


  // HIT at x = 23, y= 999
  //let mut x_vel: i32 = 23;
  //let mut y_vel: i32 = 999;
  let mut x_vel: i32 = 6;
  let mut y_vel: i32 = 9;


  let mut partb = 0;

  for x_vel in 0..1000 {
    for y_vel in -500..500 {

      let (hit, max_y) = hit(x_vel, y_vel, target_min_x, target_max_x, target_min_y, target_max_y);

      if hit == 1 {
        partb += 1;
      }
    }
  }

  println!("Part B: {}", partb);

  Ok(())
}


fn hit(mut x_vel: i32, mut y_vel: i32, target_min_x: i32, target_max_x: i32, target_min_y: i32, target_max_y: i32) -> (i32, i32) {
  let mut hit = 0;

  let mut x: i32 = 0;
  let mut y: i32 = 0;

  let mut max_y = 0;
  while hit == 0 && x <= target_max_x && y >= target_min_y {
    //println!("X: {} ; Y: {} ; xvel = {} ; yvel = {}", x, y, x_vel, y_vel);
    let (new_x, new_y, new_x_vel, new_y_vel) = step(x, y, x_vel, y_vel);
    x = new_x;
    y = new_y;
    x_vel = new_x_vel;
    y_vel = new_y_vel;

    if y > max_y { max_y = y;}

    if check(x, y, target_min_x, target_max_x, target_min_y, target_max_y) {
      //println!("X: {} ; Y: {} ; xvel = {} ; yvel = {}", x, y, x_vel, y_vel);
      //println!("HIT TARGET!");
      hit = 1;
    } else {
      //println!("X: {} ; Y: {} ; xvel = {} ; yvel = {}", x, y, x_vel, y_vel);
      //println!("no hit");
    }
  } 

  if hit == 0 {
    //println!("never hit target");
  }

  return (hit, max_y);
}
  
fn check(x: i32, y: i32, target_min_x: i32, target_max_x: i32, target_min_y: i32, target_max_y: i32) -> bool {
  //println!("{} >= {} && {} <= {} && {} <= {} && {} >= {}", x, target_min_x, x, target_max_x, y, target_max_y, y, target_min_y);
  if x >= target_min_x && x <= target_max_x && y <= target_max_y && y >= target_min_y {
    return true;
  }
  return false;
}

fn step(x: i32, y: i32, x_vel: i32, y_vel: i32) -> (i32, i32, i32, i32) {
  let new_x = x + x_vel;
  let new_y = y + y_vel;
  let mut new_x_vel = x_vel;
  let mut new_y_vel = y_vel;

  if x_vel > 0 { 
    new_x_vel -= 1; 
  } else if x_vel < 0 { 
    new_x_vel += 1;
  }

  new_y_vel -= 1;

  return (new_x, new_y, new_x_vel, new_y_vel);
  
}


