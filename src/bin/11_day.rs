use std::fs;

#[derive(Clone)]
struct Galaxy {
    x: isize,
    y: isize,
}

struct Space {
    grid: Vec<Vec<bool>>,
    rows: Vec<bool>,
    cols: Vec<bool>,
    width: usize,
    height: usize,
}

impl Space {
    fn new(contents: &str) -> Space {
        let height = contents.lines().count();
        let width = contents.lines().nth(0).unwrap().len();

        let mut grid: Vec<Vec<bool>> = vec![vec![false; width]; height];
        let mut rows: Vec<bool> = vec![false; height];
        let mut cols: Vec<bool> = vec![false; width];

        for (i, line) in contents.lines().enumerate() {
            if line.chars().all(|a| a == '.') {
                rows[i] = true;
            }
        }

        for j in 0..width {
            if contents
                .lines()
                .map(|a| a.chars().nth(j).unwrap())
                .all(|a| a == '.')
            {
                cols[j] = true;
            }
        }

        for (i, line) in contents.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                grid[i][j] = c == '#';
            }
        }

        Space {
            grid,
            rows,
            cols,
            width,
            height,
        }
    }
}

fn ans(space: &Space, exp: isize) -> i64 {
    let mut dist: Vec<Vec<Option<Galaxy>>> = vec![vec![None; space.width]; space.height];

    let mut x = 0;
    for (i, line) in space.grid.iter().enumerate() {
        let mut y = 0;
        for (j, _) in line.iter().enumerate() {
            if space.grid[i][j] {
                dist[i][j] = Some(Galaxy { x, y });
            }
            y += 1;
            if space.cols[j] {
                y += exp - 1;
            }
        }
        x += 1;
        if space.rows[i] {
            x += exp - 1;
        }
    }

    let galaxies = dist
        .clone()
        .into_iter()
        .map(|row| row.into_iter().filter(|e| e.is_some()).collect::<Vec<_>>())
        .fold(Vec::new(), |mut a, b| {
            a.extend(b);
            a
        })
        .into_iter()
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();

    let len = galaxies.len();
    let mut res = 0;

    for i in 0..len {
        for j in i + 1..len {
            res += (galaxies[i].x - galaxies[j].x).abs() + (galaxies[i].y - galaxies[j].y).abs()
        }
    }

    res as i64
}

fn part1(space: &Space) -> i64 {
    ans(&space, 2)
}

fn part2(space: &Space) -> i64 {
    ans(&space, 1000000)
}

fn main() {
    let file_path = "./input";
    let contents = fs::read_to_string(file_path).expect("[ERROR] could not read input");

    let space = Space::new(&contents);

    let part1 = part1(&space);
    println!("Part 1 answer => {part1}");

    let part2 = part2(&space);
    println!("Part 2 answer => {part2}");
}
