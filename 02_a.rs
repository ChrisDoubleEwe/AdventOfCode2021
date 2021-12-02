use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("02_in.txt")?;
    let reader = BufReader::new(file);

    let mut a_x = 0;
    let mut a_y = 0;

    let mut b_x = 0;
    let mut b_y = 0;
    let mut b_aim = 0;



    for line in reader.lines() {
        let this_line = line?;

        let dir = this_line.split(" ").nth(0).unwrap();
        let amount_str = this_line.split(" ").nth(1).unwrap();
        let amount = amount_str.parse::<i32>().unwrap();

        if dir == "forward" {
          a_x += amount;
          b_x += amount;
          b_y += b_aim * amount
        }
        if dir == "down" {
          a_y += amount;
	  b_aim += amount;
        }
        if dir == "up" {
          a_y -= amount;
          b_aim -= amount;
        }




    }
    let a_total = a_x * a_y;
    println!("Part a: {}", a_total);


    let b_total = b_x * b_y;
    println!("Part b: {}", b_total);




    Ok(())
}

