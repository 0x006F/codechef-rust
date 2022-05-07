/*
https://www.codechef.com/problems/MANGOES
 */

fn max_mangoes_can_cross(mango_weight: u8, truck_weight: u8, bridge_capacity: u8) -> u8 {
    let weight_left_for_mangoes = bridge_capacity - truck_weight;
    return weight_left_for_mangoes / mango_weight;
}

fn main() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    for _ in 0..input.trim().parse::<u16>().unwrap() {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let input: Vec<u8> = input
            .trim()
            .split(" ")
            .into_iter()
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        let max_mangoes = max_mangoes_can_cross(input[0], input[1], input[2]);
        println!("{}", max_mangoes);
    }
}
