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

    let a = i.iter();
    let b = i.iter().skip(1);
    let z = a
        .zip(b)
        .fold(0, |acc, (a, b)| if b > a { acc + 1 } else { acc });

    dbg!(z);
}
