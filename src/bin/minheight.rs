/*
https://www.codechef.com/problems/MINHEIGHT
 */

use std::io;

fn can_ride(input: &str) -> bool {
    let parsed = input.trim().split(" ").collect::<Vec<&str>>();
    return parsed[0].parse::<usize>().unwrap() >= parsed[1].parse::<usize>().unwrap();
}

fn main() {
    let mut no_of_test_cases = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut no_of_test_cases).unwrap();

    let mut test_case = String::new();

    let no_of_test_cases = no_of_test_cases.trim().parse::<usize>().unwrap();

    for _ in 0..no_of_test_cases {
        test_case.clear();
        stdin.read_line(&mut test_case).unwrap();
        let result = can_ride(&test_case);
        if result {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
