/*
https://www.codechef.com/SNCKQL21/problems/TESTSERIES
 */

use std::io;

fn main() {
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer).unwrap();
    let mut results: Vec<&str> = Vec::new();
    let test_cases = input_buffer.trim().parse::<i32>().unwrap();

    for _i in 0..test_cases {
        input_buffer.clear();
        let mut india = 0;
        let mut england = 0;
        io::stdin().read_line(&mut input_buffer).unwrap();

        let matches: Vec<&str> = input_buffer.trim().split_whitespace().collect();

        for i in matches {
            match i {
                "1" => india += 1,
                "2" => england += 1,
                _ => {}
            }
        }

        if india == england {
            results.push("draw");
            continue;
        }

        if india > england {
            results.push("india");
            continue;
        }

        if england > india {
            results.push("england");
            continue;
        }
        results.push("draw");
    }

    for i in results {
        println!("{}", i);
    }
}
