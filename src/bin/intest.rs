/*
https://www.codechef.com/problems/INTEST
*/
use std::io;

fn main() {
    let mut input_buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input_buffer).unwrap();
    let input: Vec<&str> = input_buffer.split_whitespace().collect();
    let n = input[0].parse::<i64>().unwrap();
    let k = input[1].parse::<i64>().unwrap();
    let mut o = 0;

    let mut loop_input = String::with_capacity(12);

    for _ in 0..n {
        loop_input.clear();
        stdin.read_line(&mut loop_input).unwrap();
        if loop_input.trim().parse::<i64>().unwrap() % k == 0 {
            o = o + 1;
        }
    }
    println!("{}", o);
}
