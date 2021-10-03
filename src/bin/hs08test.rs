/*
https://www.codechef.com/problems/HS08TEST
*/

use std::io::{self};

fn main() {
    let mut input_buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input_buffer).unwrap();

    let input: Vec<&str> = input_buffer.split_whitespace().collect();
    let withdraw = input[0].parse::<i64>().unwrap();
    let balance = input[1].parse::<f64>().unwrap();

    if withdraw % 5 != 0 || withdraw as f64 > balance - 0.5 {
        println!("{:.2}", balance);
        return;
    }
    println!("{:.2}", balance - withdraw as f64 - 0.5);
}
