use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;




fn main() -> io::Result<()> {
  let mut perms = Vec::new();

  for x in 0..10 {
    perms.push(0);
  }
    
  for a in 1..4 {
    for b in 1..4 {
      for c in 1..4 {
        let z = a+b+c;
        println!("{}: {}{}{}", z, a, b, c);
        perms[z]+=1;
      }
    }
  }

  for n in 3..10 {
    println!("{}: {}", n, perms[n]);
  }


  //process::exit(1);
      
  let mut p1_pos = 5-1;
  let mut p2_pos = 10-1;

  let mut p1_score = 0;
  let mut p2_score = 0;

  let mut dice = 0;
  let mut dice_rolls = 0;

  let mut turn = 0;

  //let mut history = Vec<i32>::new;
  let mut history = Vec::new();

  let mut perms1: i64 = 0;
  let mut perms2: i64 = 0;

  perms1 = 0;
  perms2 = 0;

  for n in 3..10 {
    let (new_perms1, new_perms2) = make_move(perms1, perms2, perms.clone(), p1_score, p2_score, turn, p1_pos, p2_pos, n, history.clone());
    perms1 = new_perms1;
    perms2 = new_perms2;
  }

  println!("======================");
  if perms1 > perms2 {
    println!("PART B: {:?}", perms1);
  } else {
    println!("PART B: {:?}", perms2);
  }
 
  Ok(())
}



fn make_move(mut perms1: i64, mut perms2: i64, mut perms: Vec<i32>, mut p1_score: i32, mut p2_score: i32, mut turn: i32, mut p1_pos: i32, mut p2_pos: i32, mut moves: i32 , mut history: Vec<i32> ) -> (i64, i64)  {


  history.push(moves);
  if turn == 0 {
    p1_pos = (p1_pos + moves) % 10;
    p1_score += p1_pos+1;
    //println!("Player 1 rolls {} and moves to space {} for a total score of {}", moves, p1_pos+1, p1_score);
  } else {
    p2_pos = (p2_pos + moves) % 10;
    p2_score += p2_pos+1;
    //println!("Player 2 rolls {} and moves to space {} for a total score of {}", moves, p2_pos+1, p2_score);
  }

  turn = (turn + 1) % 2;

  let winning_score = 21;

  if p1_score < winning_score && p2_score < winning_score {
    for n in 3..10 {
      let (new_perms1, new_perms2) = make_move(perms1, perms2, perms.clone(), p1_score, p2_score, turn, p1_pos, p2_pos, n, history.clone());
      perms1 = new_perms1;
      perms2 = new_perms2;
    }
    return (perms1, perms2)

  } else {
    let mut total: i64 = 1;

    for n in history.clone() {
      total = total * perms[n as usize] as i64;
    }
    if p1_score >= winning_score {
      println!("P1 Winner! {:?}  {}, {}", history.clone() , perms1+total, perms2);
      return (perms1 + total, perms2)
    } else {
      println!("P2 Winner! {:?}  {}, {}", history.clone(), perms1, perms2+total);
      return (perms1, perms2 +total)
    }
  }
}


