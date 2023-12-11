#![allow(dead_code, non_snake_case )]
use regex::Regex;
use std::{collections::HashMap, fs};

struct City<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> City<'a> {
    fn new(name: &'a str, left: &'a str, right: &'a str) -> City<'a> {
        City { name, left, right }
    }
}

fn ends_in_A(s: &str) -> bool {
    s.chars().last().unwrap() == 'A'
}

fn ends_in_Z(s: &str) -> bool {
    s.chars().last().unwrap() == 'Z'
}

fn gcd(a: i64, b: i64) -> i64 {
    if a < b {
        gcd(b, a)
    } else {
        if a % b == 0 {
            b
        } else {
            gcd(b, a % b)
        }
    }
}

fn lcm(a : i64, b: i64) -> i64 {
    (a * b) as i64 / gcd(a, b) as i64
}

fn main() {
    let file_path = "./input";
    let contents = fs::read_to_string(file_path).expect("[Error] Could not read file");

    let re = Regex::new(r"^(.+) = \((.+), (.+)\)").unwrap();
    let sequence = contents
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    let sequence_len: usize = sequence.len();

    let mut cities: HashMap<&str, City> = HashMap::new();
    for line in contents.lines().skip(2) {
        let (_, [name, left, right]) = re.captures(line).unwrap().extract();
        cities.insert(name, City::new(name, left, right));
    }

    let mut current = "AAA";
    let mut part1ans = 0;
    loop {
        if current == "ZZZ" {
            break;
        }

        match sequence.get(part1ans % sequence_len).unwrap() {
            'L' => {
                current = cities[current].left;
            }
            'R' => {
                current = cities[current].right;
            }
            _ => assert!(false, "[Error] wrong sequence"),
        }

        part1ans += 1;
    }

    println!("Part 1 ans => {part1ans}");

    let current_2: Vec<&str> = cities
        .iter()
        .filter(|(k, _)| (ends_in_A(k)))
        .map(|(&k, _)| k)
        .collect();

    let for_all = current_2
        .iter()
        .map(|&current| {
            let mut part2ans = 0;
            let mut current = current;
            loop {
                if ends_in_Z(current) {
                    break part2ans;
                }

                match sequence.get(part2ans % sequence_len).unwrap() {
                    'L' => {
                        current = cities[current].left;
                    }
                    'R' => {
                        current = cities[current].right;
                    }
                    _ => assert!(false, "[Error] wrong sequence"),
                }

                part2ans += 1;
            }
        })
        .collect::<Vec<usize>>();
    let part2ans = for_all.iter().fold(1_i64, |a, &b| lcm(a , b as i64));
    println!("Part 2 ans => {part2ans}");
}
