use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("01_in.txt")?;
    let reader = BufReader::new(file);

    let mut increases = 0;
    let mut last_num = 0;
    let mut last_but_one_num = 0;
    let mut last_window = 99999;

    for line in reader.lines() {
        let this_line = line?;
        println!("{}", this_line);
        let this_num = this_line.parse::<i32>().unwrap();

        if last_but_one_num == 0 || last_num == 0 {
          last_but_one_num = last_num;
          last_num = this_num;
          continue;
        }

        let this_window = this_num + last_num + last_but_one_num;
          
        if this_window > last_window {
          println!("Increase: {} -> {}", this_window, last_window);
          increases += 1;
   	}
        last_window = this_window;
        last_but_one_num = last_num;
        last_num = this_num;
    }
    println!("Total number of increases: {} ", increases);


    Ok(())
}

