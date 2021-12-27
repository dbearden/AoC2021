fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> String {
    let mut crabs: Vec<i32> = include_str!("input.txt")
        .split(",")
        .filter_map(|n| n.parse::<i32>().ok())
        .collect();

    let mid = crabs.len() / 2;
    let median = *crabs.select_nth_unstable(mid).1;
    crabs
        .iter()
        .map(|c| (c - median).abs())
        .sum::<i32>()
        .to_string()
}

fn part2() -> String {
    let crabs: Vec<i32> = include_str!("input.txt")
        .split(",")
        .filter_map(|n| n.parse::<i32>().ok())
        .collect();
    let mut min = i32::MAX;
    for i in 0..*crabs.iter().max().unwrap() {
        let tmp: i32 = crabs
            .iter()
            .map(|c| ((c - i).abs() * ((c - i).abs() + 1) / 2))
            .sum();
        if tmp < min {
            min = tmp;
        }
    }

    min.to_string()
}
