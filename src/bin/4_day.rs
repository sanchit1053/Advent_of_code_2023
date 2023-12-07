use std::fs;

fn part1(contents: &str) -> i32 {
    let mut ans = 0;
    for line in contents.lines() {
        let data = line.split(": ").nth(1).unwrap();
        let sep: Vec<&str> = data.split(" | ").collect();


        let ticket_num: Vec<i32> = sep[0]
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
        let value_num: Vec<i32> = sep[1]
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect();

        let mut nums = 0;
        for value in value_num {
            if ticket_num.contains(&value) {
                nums += 1;
            }
        }


        if nums >= 1 {
            ans += 2_i32.pow(nums - 1);
        }
    }
    ans
}

fn part2(contents: &str) -> i32 {
    let mut num_tickets = vec![1; contents.lines().count()];
    for (id, line) in contents.lines().enumerate() {

        let data = line.split(": ").nth(1).unwrap();
        let sep: Vec<&str> = data.split(" | ").collect();


        let ticket_num: Vec<i32> = sep[0]
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
        let value_num: Vec<i32> = sep[1]
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect();

        let mut nums = 0;
        for value in value_num {
            if ticket_num.contains(&value) {
                nums += 1;
            }
        }

        for next in id+1 .. id+nums+1 {
            num_tickets[next] += num_tickets[id];
        }
    }

    num_tickets.into_iter().sum::<i32>()
}
fn main() {
    let file_path = "./input";
    let contents = fs::read_to_string(file_path).expect("[ERROR] couldn't Read the file");

    let part1ans = part1(&contents);
    let part2ans = part2(&contents);

    println!("Part 1 Answer : {part1ans}");
    println!("Part 2 Answer : {part2ans}");
}
