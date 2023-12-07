use std::cmp;
use std::fs;
use std::ops::Range;

fn to_vec(s: &str) -> Vec<i64> {
    s.split_whitespace()
        .map(|i| i.parse::<i64>().unwrap())
        .collect()
}

struct Block {
    mappings: Vec<Vec<i64>>,
}

fn traverse(seed: i64, blocks: &Vec<Block>) -> i64 {
    let mut current = seed;
    for block in blocks {
        let mut new = current;
        for nums in block.mappings.clone() {
            if nums[1] <= current && nums[1] + nums[2] > current {
                new = nums[0] + (current - nums[1]);
                break;
            }
        }
        current = new;
    }

    current
}

fn part1(seeds: &Vec<i64>, mappings: &Vec<Block>) -> i64 {
    let mut ans = i64::MAX;

    for &seed in seeds {
        let val = traverse(seed, &mappings);
        ans = cmp::min(ans, val);
    }
    ans
}

// https://github.com/jokr-1/aoc-2023/blob/main/day-05/src/main.rs
// Reference for Option, Range
fn get_mapped<'a, 'b>(
    s: &'a Range<i64>,
    map: &'b Vec<i64>,
) -> (Option<Range<i64>>, Vec<Range<i64>>) {
    let mut remaining: Vec<Range<i64>> = Vec::new();
    let mapped: Option<Range<i64>>;
    let diff = map[0] - map[1];
    let map_start = map[1];
    let map_end = map[1] + map[2];

    if map_start >= s.end || map_end <= s.start {
        // no overlap
        remaining.push(s.clone());
        mapped = None;
    } else if map_end < s.end && map_start > s.start {
        // seed around mapping
        remaining.push(s.start..map_start);
        remaining.push(map_end..s.end);
        mapped = Some(map_start + diff..map_end + diff);
    } else if map_end >= s.end && map_start <= s.start {
        // seed in mapping
        mapped = Some(s.start + diff..s.end + diff);
    } else if map_start > s.start /* && map_start < s.end  */{
        // ending overlap
        remaining.push(s.start..map_start);
        mapped = Some(map_start + diff..s.end + diff);
    } else if /* map_end > s.start && */ map_end < s.end {
        // starting overlap
        remaining.push(map_end..s.end);
        mapped = Some(s.start + diff..map_end + diff);
    } else {
        assert!(false, "NOT REACHABLE");
        mapped = None;
    }
    (mapped, remaining)
}

fn part2(seeds: &Vec<i64>, mappings: &Vec<Block>) -> i64 {
    let mut seed_ranges: Vec<Range<i64>> = seeds.chunks(2).map(|p| p[0]..p[0] + p[1]).collect();

    for m in mappings {
        // going over each block

        let mut done: Vec<Range<i64>> = Vec::new();
        for s in m.mappings.iter() {
            let mut rem: Vec<Range<i64>> = Vec::new();
            for seed in seed_ranges.iter() {
                let (new, mut remaining) = get_mapped(&seed, s);

                if let Some(t) = new {
                    done.push(t);
                }
                rem.append(&mut remaining);
            }
            seed_ranges = rem;
        }
        done.append(&mut seed_ranges);
        seed_ranges = done;
    }
    let mut ans = i64::MAX;

    for i in seed_ranges.iter() {
        ans = cmp::min(ans, i.start);
    }
    ans
}

fn main() {
    let file_path = "./input";
    let contents = fs::read_to_string(file_path).expect("[ERROR] couldn't Read the file");
    let mut blocks = contents.split("\n\n");
    let seeds = to_vec(blocks.nth(0).unwrap().split(": ").nth(1).unwrap());

    let mut mappings: Vec<Block> = Vec::new();

    for block in blocks {
        let lines = block.lines().skip(1);
        mappings.push(Block {
            mappings: lines.map(|line| to_vec(line)).collect(),
        })
    }

    let part1ans = part1(&seeds, &mappings);
    let part2ans = part2(&seeds, &mappings);

    println!("Part 1 ans => {part1ans}");
    println!("Part 2 ans => {part2ans}");
}
