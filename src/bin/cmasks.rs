/*
https://www.codechef.com/problems/CMASKS
 */

use std::io;

fn recommend_mask(input: &str) {
    let prices = input
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    if (prices[0] * 100) < (prices[1] * 10) {
        println!("Disposable");
        return;
    }
    println!("Cloth");
}

fn main() {
    let mut n = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();
    let mut t = String::new();

    for _ in 0..n {
        t.clear();
        stdin.read_line(&mut t).unwrap();
        recommend_mask(&t);
    }
}
