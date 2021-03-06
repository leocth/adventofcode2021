#![feature(int_abs_diff)]
use adventofcode2021::prelude::*;

fn main() {
    let input = include_str!("../inputs/input7.txt");

    let inputs = input
        .trim()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .sorted()
        .collect_vec();
    let median = inputs[inputs.len() / 2];
    let fuel = inputs
        .into_iter()
        .fold(0u64, |acc, ele| acc + ele.abs_diff(median));
    dbg!(fuel);

    // code here
}
