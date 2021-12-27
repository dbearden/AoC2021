fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> String {
    let mut fishes = [0; 9];
    include_str!("input.txt")
        .split(",")
        .filter_map(|n| n.parse::<u32>().ok())
        .for_each(|f| fishes[f as usize] += 1);
    for _day in 0..80 {
        let [f0, f1, f2, f3, f4, f5, f6, f7, f8] = fishes;
        fishes = [f1, f2, f3, f4, f5, f6, f0 + f7, f8, f0];
    }

    fishes.iter().sum::<u32>().to_string()
}

fn part2() -> String {
    let mut fishes = [0u64; 9];
    include_str!("input.txt")
        .split(",")
        .filter_map(|n| n.parse::<u64>().ok())
        .for_each(|f| fishes[f as usize] += 1);
    for _day in 0..256 {
        let [f0, f1, f2, f3, f4, f5, f6, f7, f8] = fishes;
        fishes = [f1, f2, f3, f4, f5, f6, f0 + f7, f8, f0];
    }

    fishes.iter().sum::<u64>().to_string()
}
