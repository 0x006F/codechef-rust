/*
https://www.codechef.com/SNCKQL21/problems/LUCKYNUM
 */

use std::io;

fn main() {
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer).unwrap();
    let mut outs: Vec<&str> = Vec::new();

    let test_cases = input_buffer.trim().parse::<i32>().unwrap();

    for _i in 0..test_cases {
        input_buffer.clear();
        io::stdin().read_line(&mut input_buffer).unwrap();
        let arr: Vec<&str> = input_buffer.trim().split_whitespace().collect();

        if arr.contains(&"7") {
            outs.push("yes");
        } else {
            outs.push("no");
        }
    }

    for i in outs {
        println!("{}", i);
    }
}
