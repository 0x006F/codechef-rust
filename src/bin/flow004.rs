/*
https://www.codechef.com/problems/FLOW004
 */
use std::io;

fn main() {
    let mut t = String::with_capacity(5);
    let stdin = io::stdin();
    stdin.read_line(&mut t).unwrap();

    let mut outs: Vec<u32> = [].to_vec();
    let mut n = String::with_capacity(15);
    for _ in 0..t.trim().parse::<i32>().unwrap() {
        n.clear();
        stdin.read_line(&mut n).unwrap();
        let chars: Vec<char> = n.trim().chars().collect();
        outs.push(chars[0].to_digit(10).unwrap() + chars.last().unwrap().to_digit(10).unwrap());
    }

    for i in outs {
        println!("{}", i);
    }
}
