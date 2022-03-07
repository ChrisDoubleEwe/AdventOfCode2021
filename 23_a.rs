use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;

static mut low_score: i64 = 999999999;
static mut low_history: Vec<String> = Vec::new();
//static mut all_history: Vec::<(String, i32)>::new();
//static mut all_history:  <Vec<(String, i32)> as Trait>::new;
static mut all_history: Vec::<(String, i32)> = Vec::new();








fn main() -> io::Result<()> {
  let mut map_in = Vec::new();

  map_in.push("#############");
  map_in.push("#.. . . . ..#");
  map_in.push("###.#.#.#.###");
  map_in.push("###.#.#.#.###");
  map_in.push("#############");

  let mut max_x = 0;
  let mut max_y = 0;

  let mut map_array = Vec::new();


  for line in map_in {
    max_y += 1;
    if line.len() > max_x {
      max_x = line.len();
    }
    let mut map_line = Vec::new();
    for c in line.chars() {
      map_line.push(c);
    }
    map_array.push(map_line.clone());
  }

  let mut spaces = Vec::new();

  for y in 0..max_y {
    for x in 0..max_x {
      if map_array[y][x] == '.' {
        let mut pair=Vec::new();
        pair.push(y as i32);
        pair.push(x as i32);
        spaces.push(pair.clone());
      }
    }
  }

  //println!("SPACES: {:?}", spaces.clone());
 
      

  //println!("{:?}", map_array);

  let mut empty_map = Vec::new();
  let mut zeroes = Vec::new();

  for _ in 0..15 {
    zeroes.push(-9);
  }
  for _ in 0..15 {
    empty_map.push(zeroes.clone());
  }

  for y in 0..max_y {
    for x in 0..max_x {
      if map_array[y][x] == '.' || map_array[y][x] == ' ' {
        empty_map[y as usize][x as usize] = -1;
      }
    }
  }


  // Calculate distances and paths
  
  let mut map = Vec::new();
  let mut steps = Vec::new();
  let mut step = Vec::new();

  let mut paths = Vec::new();
  let mut path = Vec::new();
  let mut dummy_path = Vec::new();
  dummy_path.push(0);
  dummy_path.push(0);
  dummy_path.push(0);



  for _ in 0..15 {
    step.push(0);
    path.push(dummy_path.clone());
  }

  for _ in 0..15 {
    steps.push(step.clone());
    paths.push(path.clone());
  }


  for n in 0..spaces.len() {
    //println!(" == {} ======== ", n);
    let mut this_map = empty_map.clone();
    this_map[spaces[n][0] as usize][spaces[n][1] as usize] = 0;
    let mut keep_going = 1;
    while keep_going == 1 {
      keep_going = 0;
      for y in 1..max_y-1 {
        for x in 1..max_x-1 {
          //println!("  - x: {} ; y: {}", x, y);
          for foo in this_map.clone() {
            //println!("  mapline::: {:?}", foo);
          }
          if this_map[y][x]  == -1 {keep_going = 1;}
          if this_map[y][x] >=0 && this_map[y-1][x] == -1 {this_map[y-1][x] = this_map[y][x] + 1;}
          if this_map[y][x] >=0 && this_map[y+1][x] == -1 {this_map[y+1][x] = this_map[y][x] + 1;}
          if this_map[y][x] >=0 && this_map[y][x-1] == -1 {this_map[y][x-1] = this_map[y][x] + 1;}
          if this_map[y][x] >=0 && this_map[y][x+1] == -1 {this_map[y][x+1] = this_map[y][x] + 1;}
        }
      }
    }
    for z in 0..spaces.len() {
      steps[n][z] = this_map[spaces[z][0] as usize][spaces[z][1] as usize];  
      steps[z][n] = this_map[spaces[z][0] as usize][spaces[z][1] as usize]; 
    }
  }

  //println!("FINISHED WITH SPACES");
  // CALCULATE PATHS
  for from in 0..spaces.len() {
    for to in 0..spaces.len() {
      let mut path = Vec::new();

      let from_x = spaces[from][1];
      let from_y = spaces[from][0];
      let to_x = spaces[to][1];
      let to_y = spaces[to][0];

      // Up
      if from_y == 3 {
        path.push(coord_to_space(from_x as i32, (from_y-1) as i32, spaces.clone()));
      }

      // along corridor
      if from_x < to_x {
        for x in from_x+1..to_x+1 {
          //println!("Moving along corridor: {}", x);
          let s = coord_to_space(x as i32, 1, spaces.clone());
          if s > -1 {
            path.push(s);
          }
        }
      } 
      if from_x > to_x {
        for x in to_x..from_x {
          //println!("Moving along corridor: {}", x);
          let s = coord_to_space(x as i32, 1, spaces.clone());
          if s > -1 {
            path.push(s);
          }
        }
      } 

      // Down
      if to_y == 3 {
        path.push(coord_to_space(to_x as i32, (to_y-1) as i32, spaces.clone()));
        path.push(coord_to_space(to_x as i32, (to_y) as i32, spaces.clone()));
      }
      if to_y == 2 {
        path.push(coord_to_space(to_x as i32, (to_y) as i32, spaces.clone()));
      }
      if from_x == to_x {
        path = Vec::new();
      }

 

      //println!("Path from {} to {} :: {:?}", from, to, path);
      paths[from][to] = path.clone();

    }
  }




  

      



 

  // PRINT STEPS MATRIX
  //println!("STEPS");
  for x in 0..15 {
    for y in 0..15 {
      //println!("Steps from {} to {}   =  {}", x, y, steps[x][y]);
    }
    //println!("");
  }

  // PRINT PATHS MATRIX
  for x in 0..15 {
    for y in 0..15 {
      //println!("{:?}", paths[x][y]);
    }
  }






  let mut history = Vec::new();
  let mut score = 0;

  for _ in 0..7 {
    map.push('.');
  }
  // SET MAP
  //map.push('B');
  //map.push('C');
  //map.push('B');
  //map.push('D');
  //map.push('A');
  //map.push('D');
  //map.push('C');
  //map.push('A');

  map.push('D');
  map.push('D');
  map.push('B');
  map.push('A');
  map.push('B');
  map.push('C');
  map.push('A');
  map.push('C');





  //println!("Make first move...");
  let (score, history, map) = make_move(paths.clone(), score, history.clone(), map.clone(), steps.clone());
  //println!("SCORE: {}", score);
  //println!("MAP: {:?}", map.clone());
  //println!("MAP: {:?}", print_map(map.clone()));

  unsafe {println!("LOWEST SCORE: {}", low_score); for z in low_history.clone() { println!("  {:?}", z); }}

  Ok(())
}



fn we_have_seen(mut thing: String, my_score: i32, mut history: Vec::<(String, i32)>) -> bool {
  let mut seen = false;
  for x in history {
    if x.0 == thing && my_score >= x.1 {
      seen = true;
    } else {
    }
  }
  return seen
}

fn path_is_clear(mut from: i32, mut to: i32, mut map: Vec<char>, mut paths: Vec<Vec<Vec<i32>>>) -> bool {
  let mut return_val = true;
  for step in paths[from as usize][to as usize].clone() {
    if map[step as usize] != '.' {
      return_val = false;
    }
  }
  return return_val;
}

fn make_move(paths: Vec<Vec<Vec<i32>>>, mut score: i32, mut history: Vec<String>, mut map: Vec<char>, steps: Vec<Vec<i32>>) -> (i32, Vec<String>, Vec<char>)  {
  let map_string = print_map(map.clone());
  //if history.len() < 9 {
  //println!("MAKE MOVE: {:?}", map_string.clone());
  //}
  
  // Check to see if we're done
  unsafe {
  if (score as i64) > low_score {
    return (score, history.clone(), map.clone())
  }
  }

  if map_string == ".......+AA+BB+CC+DD" {
    //println!("DONE! With score {} - {:?} {:?}", score, history.clone(), map_string);
    for x in history.clone() {
      //println!("   {:?}", x);
    }
    unsafe {if (score as i64) < low_score {
      println!("NEW LOW: {}", score);
      low_score = score as i64;
      history.push(map_string);
      unsafe {
        for t in history.clone() {
          println!("  {:?}", t);
        }
      }
      low_history = history.clone();
    } }

      
    return (score, history.clone(), map.clone())
  }

  // Check to see if we've already been here, otherwise add to history
  unsafe {
  if we_have_seen(map_string.clone(), score, all_history.clone()) {
    //println!("already seen");
    return (score, history.clone(), map.clone())
  } else {
    let pair = (map_string.clone(), score);
    all_history.push(pair);
    //all_history.push(map_string.clone());
    history.push(map_string.clone());

  }
  }

  for piece_num in 0..map.len() {
  // Iterate over each piece
  //   make each possible move
  //     If piece in correct silo, lower berth, forget it: we're done
    if (map[piece_num] == 'A' && piece_num == 11 ) ||
       (map[piece_num] == 'B' && piece_num == 12 ) ||
       (map[piece_num] == 'C' && piece_num == 13 ) ||
       (map[piece_num] == 'D' && piece_num == 14 ) {
         continue;
    }

  //     If piece in correct silo, upper berth, and lower berth is OK, forget it: we're done
    if (map[piece_num] == 'A' && piece_num == 7 && map[11] == 'A') ||
       (map[piece_num] == 'B' && piece_num == 8 && map[11] == 'B') ||
       (map[piece_num] == 'C' && piece_num == 9 && map[11] == 'C') ||
       (map[piece_num] == 'D' && piece_num == 10 && map[11] == 'D') {
         continue;
    }

  let mut moved = 0;
  //     If piece is anywhere, and correct silo lower berth is empty & accessible, move.
    if (map[piece_num] == 'A' && map[11] == '.' && path_is_clear(piece_num as i32, 11, map.clone(), paths.clone())) {
      moved = 1;
      map[piece_num] = '.';
      map[11] = 'A';
      score += steps[piece_num][11];
      let (score, history, map) = make_move(paths.clone(), score, history.clone(), map.clone(), steps.clone());
    }
    if (map[piece_num] == 'B' && map[12] == '.' && path_is_clear(piece_num as i32, 12, map.clone(), paths.clone())) {
      moved = 1;
      map[piece_num] = '.';
      map[12] = 'B';
      score += 10*steps[piece_num][12];
      let (score, history, map) = make_move(paths.clone(), score, history.clone(), map.clone(), steps.clone());
    }
    if (map[piece_num] == 'C' && map[13] == '.' && path_is_clear(piece_num as i32, 13, map.clone(), paths.clone())) {
      moved = 1;
      map[piece_num] = '.';
      map[13] = 'C';
      score += 100*steps[piece_num][13];
      let (score, history, map) = make_move(paths.clone(), score, history.clone(), map.clone(), steps.clone());
    }
    if (map[piece_num] == 'D' && map[14] == '.' && path_is_clear(piece_num as i32, 14, map.clone(), paths.clone())) {
      moved = 1;
      map[piece_num] = '.';
      map[14] = 'D';
      score += 1000*steps[piece_num][14];
      //println!("piece is anywhere, and correct silo lower berth is empty & accessible");
      let (score, history, map) = make_move(paths.clone(), score, history.clone(), map.clone(), steps.clone());
    }

  //     If piece is anywhere, and correct silo upper berth is E&A, and lower silo has correct occupant, move
    if (map[piece_num] == 'A' && map[11] == 'A' && map[7] == '.' && path_is_clear(piece_num as i32, 7, map.clone(), paths.clone())) {
      moved = 1;
      map[piece_num] = '.';
      map[7] = 'A';
      score += steps[piece_num][7];
      let (score, history, map) = make_move(paths.clone(), score, history.clone(), map.clone(), steps.clone());
    }
    if (map[piece_num] == 'B' && map[12] == 'B' && map[8] == '.' && path_is_clear(piece_num as i32, 8, map.clone(), paths.clone())) {
      moved = 1;
      map[piece_num] = '.';
      map[8] = 'B';
      score += 10*steps[piece_num][8];
      let (score, history, map) = make_move(paths.clone(), score, history.clone(), map.clone(), steps.clone());
    }
    if (map[piece_num] == 'C' && map[13] == 'C' && map[9] == '.' && path_is_clear(piece_num as i32, 9, map.clone(), paths.clone())) {
      moved = 1;
      map[piece_num] = '.';
      map[9] = 'C';
      score += 100*steps[piece_num][9];
      let (score, history, map) = make_move(paths.clone(), score, history.clone(), map.clone(), steps.clone());
    }
    if (map[piece_num] == 'D' && map[14] == 'D' && map[10] == '.' && path_is_clear(piece_num as i32, 10, map.clone(), paths.clone())) {
      moved = 1;
      map[piece_num] = '.';
      map[10] = 'D';
      score += 1000*steps[piece_num][10];
      //println!("piece is anywhere, and correct silo upper berth is E&A, and lower silo has correct occupant");
      let (score, history, map) = make_move(paths.clone(), score, history.clone(), map.clone(), steps.clone());
    }


  //     If piece is in silo, move to all available coridor spaces
    if moved == 0 {
      for move_to in 0..7 {
        if map[piece_num] != '.' && piece_num > 6 && map[move_to] == '.' && path_is_clear(piece_num as i32, move_to as i32, map.clone(), paths.clone()) {
          let mut new_map = map.clone();
          new_map[piece_num] = '.';
          new_map[move_to] = map[piece_num];
          let mut multiplier = 1;
          if map[piece_num] == 'B' {multiplier = 10;}
          if map[piece_num] == 'C' {multiplier = 100;}
          if map[piece_num] == 'D' {multiplier = 1000;}

          let add_steps = steps[piece_num][move_to] * multiplier;
          //println!("Moving from {} to {}", piece_num, move_to);
          let (score, history, map) = make_move(paths.clone(), score+add_steps, history.clone(), new_map.clone(), steps.clone());
        }
      }
    } 
  }

  // If no move possible, abort
  (score, history.clone(), map.clone())
}

fn print_map(map: Vec<char>) -> String {
  let mut val = String::from("");
  for n in 0..7 {
    val.push(map[n]);
  }
  val.push('+');
  val.push(map[7]);
  val.push(map[11]);
  val.push('+');
  val.push(map[8]);
  val.push(map[12]);
  val.push('+');
  val.push(map[9]);
  val.push(map[13]);
  val.push('+');
  val.push(map[10]);
  val.push(map[14]);

  (val.clone())
}

fn coord_to_space(x: i32, y: i32, spaces: Vec<Vec<i32>>) -> (i32) {
  //println!("coord to space! x={}, y={}", x, y);
  //println!("  {:?}", spaces);
  for s in 0..spaces.len() {
    //println!("Checking against x={}, y={}", spaces[s][1], spaces[s][0]);
    if spaces[s][0] == y && spaces[s][1] == x {
      //println!("coord x={} ; y={} is space {}", x, y, s);
      return s as i32;
    }
  }
  (-1)
} 

