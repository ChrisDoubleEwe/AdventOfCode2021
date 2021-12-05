use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("04_in.txt")?;
    let reader = BufReader::new(file);

    let mut first_line = String::new();
    let mut count = 0;

    let mut card = Vec::<(i32,bool)>::new();
    let mut cards = Vec::new();


    for line in reader.lines() {
      count += 1;
      let this_line = line?;
      if this_line == "" {
        if card.len() > 1 {
          cards.push(card);
        }
        card = Vec::<(i32,bool)>::new();
        continue;
      }
      if count == 1 {
        first_line = this_line.clone();
      } else {
        let line_numbers: Vec<i32> = this_line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        for num in line_numbers {
          let pair = (num, false);
          card.push(pair);
        }
      }
    }
    cards.push(card);


    let numbers: Vec<i32> = first_line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();


    let mut winners = 0;
    for n in numbers {
      cards = cross_off(&cards, n);
      let score = check_winner(&cards);
      if score > -1 {
        let total = score * n;
        if winners == 0 {
          println!("PART A: {}", total);
          winners += 1;
        }

        cards = remove_winner(&cards);
        if cards.len() == 0 {
          println!("PART B: {}", total);
          break;
        }
      }
    }

    Ok(())
}

fn cross_off(c: &Vec<Vec<(i32,bool)>>, n: i32) -> Vec<Vec<(i32,bool)>> {
  let mut new_cards = Vec::new();

  for card in c {
    let mut new_card = Vec::<(i32,bool)>::new();
    for pair in card {
      let mut new_pair = (pair.0, pair.1);
      if pair.0 == n {
        new_pair.1 = true;
      }
      new_card.push(new_pair);
    }
    new_cards.push(new_card);
  }
  return new_cards;
}


fn remove_winner(c: &Vec<Vec<(i32,bool)>>) -> Vec<Vec<(i32,bool)>> {
  let mut new_cards = Vec::new();

  for card in c {
    let mut check_single_deck = Vec::new();
    let mut new_card = Vec::<(i32,bool)>::new();

    for pair in card {
      let mut new_pair = (pair.0, pair.1);
      new_card.push(new_pair);
    }

    check_single_deck.push(new_card.clone());
    let score = check_winner(&check_single_deck);
    if score == -1 {
      new_cards.push(new_card);
    }

  }

  //for card in c {
    //let mut check_single_deck = Vec::new();
    //check_single_deck.push(c);
    //let score = check_winner(&check_single_deck);
    //if score == -1 {
    //new_cards.push(c);
    //}
  //}
  return new_cards;
}


fn print_cards(c: &Vec<Vec<(i32,bool)>>) {
  let mut n = 0;
  for card in c {
    n += 1;
    let mut col = 0;
    let mut row_str = String::new(); 
    for num in card {
      col += 1;
      let num_str = format!("{:02}", num.0.to_string());
      let mut num_got_str = String::new();
      if num.1 {
        num_got_str += "*";
        num_got_str += &num_str;
        num_got_str += "*";
      } else {
        num_got_str += " ";
        num_got_str += &num_str;
        num_got_str += " ";
      }
      row_str += &num_got_str;
      row_str += "  ";
      if col == 5 {
        row_str = String::new();
        col = 0;
      }
    }
  }
}

fn check_winner(c: &Vec<Vec<(i32,bool)>>) -> i32{
  // CHECK HORIZONTAL
  for card in c {
    for index in 0..5 {
      let mut line = 0;
      let start = index * 5;
      let finish = start + 5;
 
      for p in start..finish {
        if card[p].1 {
          line += 1;
        }
      }
      if line == 5 {
        return score(&card);
      }
    }
  }

  // CHECK VERTICAL
  for card in c {
    for index in 0..5 {
      let mut line = 0;
      if card[index].1 {
        line += 1;
      }
      if card[index+5].1 {
        line += 1;
      }
      if card[index+10].1 {
        line += 1;
      }
      if card[index+15].1 {
        line += 1;
      }
      if card[index+20].1 {
        line += 1;
      }
      if line == 5 {
        return score(&card);
      }
    }
  }
  return -1;
}

fn score(c: &Vec<(i32,bool)>) -> i32{
  let mut s = 0;
  for pair in c {
    if !pair.1 {
      s += pair.0
    }
  }
  return s;
}


