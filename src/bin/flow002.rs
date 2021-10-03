/*
https://www.codechef.com/problems/FLOW002
*/
use std::io;

/* [NOT SUBMITTED DUE TO UNAVAILABILITY OF RUST] */
fn main() {
    let mut test_case_buffer = String::with_capacity(4);
    let stdin = io::stdin();
    stdin.read_line(&mut test_case_buffer).unwrap();
    let mut outs: Vec<i32> = [].to_vec();

    let mut prob_buffer = String::with_capacity(11);
    for _ in 0..test_case_buffer.trim().parse::<i32>().unwrap() {
        prob_buffer.clear();
        stdin.read_line(&mut prob_buffer).unwrap();
        let operands: Vec<&str> = prob_buffer.trim().split_whitespace().collect();
        outs.push(operands[0].parse::<i32>().unwrap() % operands[1].parse::<i32>().unwrap());
    }
    for i in outs {
        println!("{}", i);
    }
}
