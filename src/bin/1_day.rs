use std::fs;

fn first(line: &String) -> u32 {
    for c in line.chars() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
    }
    0
}

fn last(line: &String) -> u32 {
    for c in line.chars().rev() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
    }
    0
}

fn parse_string(line: &str) -> Vec<String> {
    line.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.to_string()
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("seven", "seven7seven")
                .replace("six", "six6six")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .collect()
}

fn main() {
    let file_path = "./input";
    let contents = fs::read_to_string(file_path).expect("[ERROR] couldn't Read the file");
    let contents = parse_string(&contents);

    let mut ans = 0;
    for line in contents {
        let first_digit = first(&line);
        let last_digit = last(&line);
        let num = 10 * first_digit + last_digit;
        ans = ans + num;
        println!("{line} -> {num}");
    }

    println!("ANSWER IS  {ans}")
}
