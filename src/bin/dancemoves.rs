/*
https://www.codechef.com/SNCK1A21/problems/DANCEMOVES
 */

use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let t = buffer.trim().parse::<i32>().unwrap();

    for _i in 0..t {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let c: Vec<i32> = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let mut chef = c[0];
        let partner = c[1];
        let mut steps = 0;

        loop {
            if chef == partner {
                break;
            }

            if chef < partner {
                chef += 2;
                steps += 1;
            }

            if chef > partner {
                chef -= 1;
                steps += 1;
            }
        }
        println!("{}", steps);
    }
}
