/*
https://www.codechef.com/problems/FCTRL2
 */

use std::io;

fn fac(n: usize) -> usize {
    if n == 0 || n == 1 {
        return 1;
    }
    return n * fac(n - 1);
}

fn main() {
    let mut t = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut t).unwrap();

    let mut n = String::new();
    for _ in 0..t.trim().parse::<usize>().unwrap() {
        n.clear();
        stdin.read_line(&mut n).unwrap();
        println!("{}", fac(n.trim().parse::<usize>().unwrap()));
    }
}
