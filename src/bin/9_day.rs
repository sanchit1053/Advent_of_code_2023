use std::fs;
fn to_vec(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

fn is_all_same<T : Eq + Ord>(v : &[T]) -> bool {
    v.iter().min() == v.iter().max()
}

fn part1_solve(s : &str) -> i32 {
    let mut vec = to_vec(s);
    let mut last : Vec<i32> = Vec::new();

    last.push(vec.last().unwrap().to_owned());

    while !is_all_same(&vec) {
        vec = vec.windows(2).map(|w| w[1] - w[0]).collect();
        last.push(vec.last().unwrap().to_owned());
    }

    last.iter().sum()
}

fn part2_solve(s : &str) -> i32 {
    let mut vec = to_vec(s);
    let mut first : Vec<i32> = Vec::new();

    first.push(vec.first().unwrap().to_owned());

    while !is_all_same(&vec) {
        vec = vec.windows(2).map(|w| w[1] - w[0]).collect();
        first.push(vec.first().unwrap().to_owned());
    }

    first.iter().rev().fold(0, |a, b| b - a) 
}

fn main() {
    let file_path = "./input";
    let contents = fs::read_to_string(file_path).expect("[Error] Could not read file");

    let part1 : i32 = contents.lines().map(part1_solve).sum();
    println!("Part 1 ans {}", part1);

    let part2 : i32 = contents.lines().map(part2_solve).sum();
    println!("Part 2 ans {}", part2);
}
