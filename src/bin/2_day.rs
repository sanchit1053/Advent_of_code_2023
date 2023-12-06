use std::cmp;
use std::fs;

#[derive(Debug)]
struct Block {
    red: i32,
    green: i32,
    blue: i32,
}

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

impl Block {
    fn parse(s: &str) -> Block {
        let blocks = s.split(", ");
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for block in blocks {
            let temp: Vec<&str> = block.split(" ").collect();
            match temp[1] {
                "red" => {
                    red = temp[0].parse().expect("{temp[1]}");
                }
                "blue" => {
                    blue = temp[0].parse().expect("{temp[1]}");
                }
                "green" => {
                    green = temp[0].parse().expect("{temp[1]}");
                }
                _ => {
                    assert!(false, "Unreachable");
                }
            }
        }
        Block { red, green, blue }
    }

    fn possible(&self) -> bool {
        if self.red > RED {
            false
        } else if self.green > GREEN {
            false
        } else if self.blue > BLUE {
            false
        } else {
            true
        }
    }
}

fn main() {
    let file_path = "./input";
    let contents = fs::read_to_string(file_path).expect("[ERROR] couldn't Read the file");

    let lines = contents.lines();

    let mut ans = 0;
    for line in lines {
        let game: Vec<&str> = line.split(": ").collect();
        let num: Vec<&str> = game[0].split(" ").collect();
        let turns = game[1].split("; ");

        let blocks = turns.map(Block::parse);
        let min = blocks.fold((0, 0, 0), |one, two| {
            (
                cmp::max(one.0, two.red),
                cmp::max(one.1, two.green),
                cmp::max(one.2, two.blue)
            )
        });
        ans += min.0 * min.1 * min.2;
    }
    println!("{ans}");
}
