#![allow(non_snake_case)]
use std::fs;
use std::iter::zip;

fn to_vec(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

fn without_space(contents: &str) -> (i64, i64) {
    let time = contents
        .lines()
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    let dist = contents
        .lines()
        .nth(1)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    (time, dist)
}

#[derive(Debug)]
struct Race {
    time: i32,
    dist: i32,
}

fn main() {
    let file_path = "./input";
    let contents = fs::read_to_string(file_path).expect("[ERROR] couldn't Read the file");

    let mut races: Vec<Race> = Vec::new();
    let times = to_vec(contents.lines().nth(0).unwrap().split(":").nth(1).unwrap());
    let dist = to_vec(contents.lines().nth(1).unwrap().split(":").nth(1).unwrap());

    for (t, d) in zip(times, dist) {
        races.push(Race { time: t, dist: d })
    }

    let possible = races.iter().map(|Race { time: t, dist: d }| {
        let D: f64 = (t * t - 4 * d) as f64;
        let D = D.sqrt();
        let lower: i64 = ((*t as f64 - D) / 2_f64 + 0.0000001).ceil() as i64;
        let upper: i64 = ((*t as f64 + D) / 2_f64 - 0.0000001).floor() as i64;
        upper - lower + 1
    });

    let part1ans: i64 = possible.product();
    println!("Part 1 ans => {part1ans}");

    let (time, dist) = without_space(&contents);

    let D: f64 = (time * time - 4 * dist) as f64;
    let D = D.sqrt();
    let lower: i64 = ((time as f64 - D) / 2_f64 + 0.0000001).ceil() as i64;
    let upper: i64 = ((time as f64 + D) / 2_f64 - 0.0000001).floor() as i64;
    let part2ans = upper - lower + 1;

    println!("Part 2 ans => {part2ans}");
}
