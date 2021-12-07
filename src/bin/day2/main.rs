use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Direction {
    o: Orientation,
    m: i32,
}

enum Orientation {
    Forward,
    Down,
    Up,
}

struct Position {
    h: i32,
    d: i32,
    a: i32,
}

fn main() {
    let input = "../inputs/day2/input.txt";
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    let directions: Vec<Direction> = get_directions(input);
    let mut pos = Position { h: 0, d: 0, a: 0 };
    for d in directions {
        match d.o {
            Orientation::Forward => pos.h += d.m,
            Orientation::Down => pos.d += d.m,
            Orientation::Up => pos.d -= d.m,
        }
    }

    pos.h * pos.d
}

fn part_2(input: &str) -> i32 {
    let directions: Vec<Direction> = get_directions(input);
    let mut pos = Position { h: 0, d: 0, a: 0 };
    for d in directions {
        match d.o {
            Orientation::Forward => {
                pos.h += d.m;
                pos.d += pos.a * d.m;
            }
            Orientation::Down => pos.a += d.m,
            Orientation::Up => pos.a -= d.m,
        }
    }

    pos.h * pos.d
}

fn get_directions(input: &str) -> Vec<Direction> {
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    reader
        .lines()
        .filter_map(|n| {
            let direction = n.unwrap();
            let o = direction.split_whitespace().next().unwrap();
            let o = match o {
                "forward" => Orientation::Forward,
                "down" => Orientation::Down,
                "up" => Orientation::Up,
                _ => panic!(),
            };
            Some(Direction {
                o,
                m: direction
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap(),
            })
        })
        .collect()
}
