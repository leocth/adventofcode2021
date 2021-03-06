use adventofcode2021::prelude::*;

fn main() {
    let input = include_str!("../inputs/input2.txt").split_whitespace();

    let (depth, _, hposition) = input.tuples().fold(
        (0, 0, 0),
        |(mut depth, mut aim, mut hposition), (dir, amount)| {
            let amount = str::parse::<u32>(amount).unwrap();
            match dir {
                "forward" => {
                    hposition += amount;
                    depth += aim * amount;
                }
                "up" => aim -= amount,
                "down" => aim += amount,
                _ => unreachable!(),
            }
            (depth, aim, hposition)
        },
    );

    dbg!(depth * hposition);
    // code here
}
