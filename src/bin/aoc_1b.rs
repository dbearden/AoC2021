use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input1.txt").unwrap();
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

    dbg!(z);
}
