use std::fs;

fn main() {
    let mut v = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .into_iter()
        .map(|x| {
            x.split("\n")
                .into_iter()
                .map(|num| num.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    v.sort();
    println!("Part 1: {}", v.last().unwrap());
    println!("Part 2: {}", v.iter().rev().take(3).sum::<u32>());
}
