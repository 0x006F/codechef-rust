/*
* https://www.codechef.com/problems/FLOW006
 */

use std::io;
fn main() {
    let mut test_case_no_buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut test_case_no_buffer).unwrap();
    let mut result_arr: Vec<u32> = [].to_vec();

    let mut prob_buffer = String::new();
    for _ in 0..test_case_no_buffer.trim().parse::<i64>().unwrap() {
        prob_buffer.clear();

        stdin.read_line(&mut prob_buffer).unwrap();

        let mut result = 0;
        for i in prob_buffer.trim().chars() {
            result = result + i.to_digit(10).unwrap_or(0);
        }
        result_arr.push(result);
    }
    for item in result_arr {
        println!("{}", item);
    }
}
