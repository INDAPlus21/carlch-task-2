use std::io;
use std::io::prelude::*;

fn main() {
    // Get stream
    let stream = io::stdin();
 
    // Seperate each line 
    let mut lines = stream
                    .lock()
                    .lines()
                    .map(|_line| _line
                        .ok()
                        .unwrap());
    
    // Get the first line and parse
    let number_count = lines
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    /* Seperate each number into segments and parse them to integers
     * and add them into a vector */
    let mut numbers = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|_number| _number
            .parse::<usize>()
            .unwrap())
        .collect::<Vec<usize>>();

    // Sort the vector into ascending order
    numbers.sort();

    /* Use different maths to find the amount of numbers
     * to add depending on if there is an even or uneven 
     * amount of numbers */
    let a_sum = 
        if number_count % 2 == 0 {
            number_count/2
        } else {
            (number_count-1)/2 
        };
    
    // Add the largest half of all numbers 
    let mut sum = 0;

    for _index in 0..number_count {
        if _index >= a_sum {
            sum += numbers[_index];            
        }
    };

    // Print out the sum
    println!("{}", sum);
}
