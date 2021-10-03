/*
* https://www.codechef.com/problems/FLOW001
 */

use std::io;

fn main() {
    let mut test_case_buffer = String::with_capacity(4);
    let stdin = io::stdin();
    stdin.read_line(&mut test_case_buffer).unwrap();

    let mut prob_input_buf = String::with_capacity(11);
    for _ in 0..test_case_buffer.trim().parse::<i32>().unwrap() {
        prob_input_buf.clear();
        stdin.read_line(&mut prob_input_buf).unwrap();
        let prob: Vec<&str> = prob_input_buf.trim().split_whitespace().collect();
        println!(
            "{}",
            prob[0].parse::<i32>().unwrap() + prob[1].parse::<i32>().unwrap()
        );
    }
}
