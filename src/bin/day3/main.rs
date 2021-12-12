use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "inputs/day3/input.txt";
fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    let f = File::open(input).unwrap();
    let r = BufReader::new(f);

    let i: Vec<String> = r.lines().filter_map(|l| l.ok()).collect();
    let width = i[0].len();
    let count = i.len();
    let mut gamma = vec![0; width];
    for line in i {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                gamma[i] += 1;
            }
        }
    }

    let mut res = 0;
    for d in gamma {
        if d > count / 2 {
            res = (res << 1) + 1;
        } else {
            res = res << 1;
        }
    }

    let epsilon = !res & (1 << width) - 1;
    println!("Gamma: {:#b}, Epsilon: {:#b}", res, epsilon);
    res * epsilon
}

fn part2(input: &str) -> u32 {
    let f = File::open(input).unwrap();
    let r = BufReader::new(f);

    let selector = 0b1 << 11;
    let (o, c): (Vec<u32>, Vec<u32>) = r
        .lines()
        .filter_map(|l| l.ok().map(|n| u32::from_str_radix(&n, 2).ok()).flatten())
        .partition(|n| *n & selector == selector);

    let (o, c) = if o.len() < c.len() { (c, o) } else { (o, c) };

    let oxygen = get_oxygen_rating(o, selector >> 1);
    let co2 = get_co2_rating(c, selector >> 1);
    println!("{}, {}", oxygen, co2);
    oxygen * co2
}

fn get_oxygen_rating(set: Vec<u32>, selector: u32) -> u32 {
    if set.len() == 1 {
        return set[0];
    }

    let reduced = {
        let p: (Vec<u32>, Vec<u32>) = set.iter().partition(|n| *n & selector == selector);
        if p.0.len() >= p.1.len() {
            p.0
        } else {
            p.1
        }
    };
    return get_oxygen_rating(reduced, selector >> 1);
}

fn get_co2_rating(set: Vec<u32>, selector: u32) -> u32 {
    if set.len() == 1 {
        return set[0];
    }

    let reduced = {
        let p: (Vec<u32>, Vec<u32>) = set.iter().partition(|n| *n & selector != selector);
        if p.0.len() <= p.1.len() {
            p.0
        } else {
            p.1
        }
    };
    return get_co2_rating(reduced, selector >> 1);
}
