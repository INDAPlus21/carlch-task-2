use std::io;
use std::io::prelude::*;
use std::cmp::*;

fn main() {
    // Get stream
    let stream = io::stdin();

    // Seperate each number
    let mut lines = stream
                    .lock()
                    .lines()
                    .map(|_line| _line
                        .ok()
                        .unwrap());
  
    // Collect inputs in a vector
    let inputs = lines
                 .next()
                 .unwrap()
                 .split_whitespace()
                 .map(|_number| _number
                     .parse::<usize>()
                     .unwrap())
                 .collect::<Vec<usize>>();

    // Determine which number to print at what position
    for _xindex in 0..inputs[0] {
        for _yindex in 0..inputs[1] {
            let val = min(min(_xindex, _yindex), min(inputs[0] - _xindex - 1, inputs[1] - _yindex - 1)) + 1;
           
            // If the value is larger than 9, replace it with "."
            if val > 9 {
                print!("{}", ".");
            } else {
                print!("{}", val);
            }
        }
        println!();
    }
}
