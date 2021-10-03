/*
https://www.codechef.com/problems/LUCKFOUR
 */

use std::io;

fn main() {
    let mut t = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut t).unwrap();

    for _ in 0..t.trim().parse::<i32>().unwrap() {
        let mut count = 0;
        let mut n = String::new();
        stdin.read_line(&mut n).unwrap();

        let mut num = n.trim().parse::<i32>().unwrap();

        while num > 0 {
            if num % 10 == 4 {
                count = count + 1;
            }
            num = num / 10
        }

        println!("{}", count);
    }
}
