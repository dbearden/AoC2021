use std::{io::{BufReader, BufRead, Read}, fs::File};

const INPUT: &str = "./inputs/day4/input.txt";
fn main(){
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}

#[derive(Debug)]
struct Board{
    squares: Vec<(u32,bool)>
}

impl Board{
    fn mark(&mut self, mv: u32) {
        for s in &mut self.squares {
            if s.0 == mv{
                s.1 = true;
            }
        }
    }

    fn check(&self, mv: u32) -> Option<u32> {
        for i in 0..5{
            if self.squares[5*i..5*i+5].iter().all(|s| s.1) || [self.squares[i], self.squares[5+i], self.squares[10+i], self.squares[15+i], self.squares[20+i]].iter().all(|s| s.1){
                let sum = self.squares.iter().fold(0, |acc, s| if !s.1 {acc + s.0} else{ acc});
                let res = mv * sum;
                return Some(res);
            }
        }

        None
    }
}
fn part1(input:&str)-> u32{
    let f = File::open(input).unwrap();
    let mut r = BufReader::new(f);

    let mut moves: String = String::new();
    r.read_line(&mut moves).unwrap();
    let moves: Vec<u32> = moves.split(',').filter_map(|n| n.parse::<u32>().ok()).collect();
    let mut boards = String::new();
    r.read_to_string(&mut boards).unwrap();
    let mut boards: Vec<_> = boards.split("\n\n").map(|l|{
        Board{
            squares: l.split_whitespace().filter_map(|n| Some((n.parse().unwrap(), false))).collect()
        }
    }).collect();
    for m in moves{
        for b in &mut boards{
            b.mark(m);
            if let Some(score) = b.check(m){return score;}
        }
    }

    0
}

fn part2(input: &str) -> u32 {
    
    let f = File::open(input).unwrap();
    let mut r = BufReader::new(f);

    let mut moves: String = String::new();
    r.read_line(&mut moves).unwrap();
    let moves: Vec<u32> = moves.split(',').filter_map(|n| n.parse::<u32>().ok()).collect();
    let mut boards = String::new();
    r.read_to_string(&mut boards).unwrap();
    let mut boards: Vec<_> = boards.split("\n\n").map(|l|{
        Board{
            squares: l.split_whitespace().filter_map(|n| Some((n.parse().unwrap(), false))).collect()
        }
    }).collect();

    let mut moves = moves.iter();
    while boards.len() > 1{
        let m = moves.next().unwrap();
        boards.iter_mut().for_each(|b| b.mark(*m));
        boards = boards.into_iter().filter(|b| b.check(*m).is_none()).collect();
    }

    while let score ={ let m = moves.next().unwrap();
        boards[0].mark(*m);
        boards[0].check(*m)
    }{
        let score = score;
        match score {
            Some(score) => return score,
            None => continue
        }
    }

    0
}