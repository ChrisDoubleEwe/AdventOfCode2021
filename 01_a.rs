use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("01_in.txt")?;
    let reader = BufReader::new(file);

    let mut increases = 0;
    let mut last_num = 999999;
    for line in reader.lines() {
        let this_line = line?;
        println!("{}", this_line);
        let this_num = this_line.parse::<i32>().unwrap();
        if this_num > last_num {
          println!("Increase: {} -> {}", this_num, last_num);
          increases += 1;
   	}
        last_num = this_num;
    }
    println!("Total number of increases: {} ", increases);


    Ok(())
}

