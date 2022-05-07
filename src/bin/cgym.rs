/*
https://www.codechef.com/problems/CGYM
 */

fn can_join_gym(budget: u8, gym_fee: u8, trainer_fee: u8) -> u8 {
    if gym_fee > budget {
        return 0;
    }

    if gym_fee + trainer_fee > budget {
        return 1;
    }
    return 2;
}

fn main() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    for _ in 0..input.trim().parse::<u8>().unwrap() {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let input = input
            .trim()
            .split(" ")
            .into_iter()
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("{}", can_join_gym(input[2], input[0], input[1]));
    }
}
