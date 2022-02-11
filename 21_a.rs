use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;




fn main() -> io::Result<()> {
  let mut p1_pos = 5-1;
  let mut p2_pos = 10-1;

  let mut p1_score = 0;
  let mut p2_score = 0;

  let mut dice = 0;
  let mut dice_rolls = 0;

  let mut turn = 0;

  while (p1_score < 1000) && (p2_score < 1000) {
    let mut moves = 0;
    for _ in 0..3 {
      dice_rolls += 1;
      dice += 1;
      if dice == 101 {dice = 1;}
      moves += dice;
    }

    if turn == 0 {
      p1_pos = (p1_pos + moves) % 10;
      p1_score += p1_pos+1;
      println!("Player 1 rolls {} and moves to space {} for a total score of {}", moves, p1_pos+1, p1_score);
    } else {
      p2_pos = (p2_pos + moves) % 10;
      p2_score += p2_pos+1;
      println!("Player 2 rolls {} and moves to space {} for a total score of {}", moves, p2_pos+1, p2_score);
    }

    turn = (turn + 1) % 2
  }

  let mut losing_score = 0;
  if turn == 0 {
    losing_score = p1_score;
  } else {
    losing_score = p2_score;
  }

  let mut result = losing_score * dice_rolls;

  println!("PART A: {}", result);
    
  Ok(())
}


