use std::{
    fs::File,
    io::{BufRead, BufReader},
};
fn main() {
    println!("Part 1: {}", part_1("inputs/day1/input1.txt"));
    println!("Part 2: {}", part_2("inputs/day1/input1.txt"));
}

fn part_1(input: &str) -> i32 {
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    let i: Vec<i32> = reader
        .lines()
        .filter_map(|n| n.unwrap().parse::<i32>().unwrap().into())
        .collect();

    let a = i.iter();
    let b = i.iter().skip(1);
    let z = a
        .zip(b)
        .fold(0, |acc, (a, b)| if b > a { acc + 1 } else { acc });

    z
}

fn part_2(input: &str) -> i32 {
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    let i: Vec<i32> = reader
        .lines()
        .filter_map(|n| n.unwrap().parse::<i32>().unwrap().into())
        .collect();

    let a = i.windows(3);
    let b = i.windows(3).skip(1);
    let z = a.zip(b).fold(0, |acc, (a, b)| {
        if b.iter().sum::<i32>() > a.iter().sum() {
            acc + 1
        } else {
            acc
        }
    });

    z
}
