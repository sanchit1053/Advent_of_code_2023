use std::fs;
use std::cmp;

struct Grid {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

struct Pos {
    x: usize,
    y: usize,
    size: usize,
    number : i32,
}
impl Grid {
    fn new(input: &str) -> Grid {
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut height = 0;
        let mut width = 0;
        for line in input.lines() {
            grid.push(line.chars().collect());
            height += 1;
            width = grid[0].len();
        }

        Grid {
            grid,
            height,
            width,
        }
    }

    fn get_numbers(&self) -> Vec<Pos> {
        let mut res: Vec<Pos> = Vec::new();
        let mut line_number = 0;
        for line in &self.grid {
            let mut x = 0;
            while x < self.width {
                let start = x;
                while x < self.width {
                    if line[x].is_digit(10) {
                        x += 1;
                    }
                    else {
                        break;
                    }
                }
                let end = x;

                if end > start {
                    res.push(Pos {
                        x: line_number,
                        y: start,
                        size: end - start,
                        number : self.grid[line_number][start..end].into_iter().collect::<String>().parse().unwrap(),
                    });
                }

                x += 1;
            }
            line_number += 1;
        }
        res
    }

    fn get_symbols(&self) -> Vec<Pos> {
        let mut res: Vec<Pos> = Vec::new();
        let mut line_number = 0;
        for line in &self.grid {
            let mut x = 0;
            while x < self.width {
                if !(line[x].is_digit(10) || line[x] == '.') {
                    res.push(Pos {
                        x: line_number,
                        y: x,
                        size: 1,
                        number : 0,
                    });
                }
                x += 1;
            }
            line_number += 1;
        }
        res
    }
}

fn part1(g : &Grid, nums: &Vec<Pos>) -> i32 {

    let mut ans = 0;
    for pos in nums {
        let top = cmp::max(0, pos.x as i32 - 1);
        let bottom = cmp::min(g.height as i32 - 1, pos.x as i32 + 1);
        let start = cmp::max(0, pos.y as i32 - 1);
        let end = cmp :: min(g.width - 1, pos.y + pos.size + 1) as i32;

        for i in top..bottom + 1 {
            if g.grid[i as usize][start as usize..end as usize].iter().any(|c| ! (c.is_digit(10) || *c == '.')) {
                ans += pos.number;
                break;
            }
        }
    }
    ans

}

fn part2(g : &Grid, nums: &Vec<Pos>) -> i32 {
    let mut ans = 0;
    for x in 1..g.height{

        for y in 1..g.width{
            if g.grid[x][y] != '*' {
                continue;
            }

            let mut near = Vec::new();

            for pos in nums {
                if (pos.x as i32 - x as i32).abs() <= 1 {
                    if y as i32 >= pos.y as i32 - 1 as i32 && y <= pos.y + pos.size {
                        near.push(pos.number);
                    }
                }
            }

            if near.len() == 2 {
                println!("{x}, {y}");
                ans += near[0] * near[1];
            }
        }
    }
    ans
}

fn main() {
    let file_path = "./input";
    let contents = fs::read_to_string(file_path).expect("[ERROR] couldn't Read the file");

    let m = Grid::new(&contents);
    let nums = m.get_numbers();
    // let sym = m.get_symbols();

    let part1_ans = part1(&m, &nums);
    let part2_ans = part2(&m, &nums);

    println!("{}", part1_ans);
    println!("{}", part2_ans);
}
