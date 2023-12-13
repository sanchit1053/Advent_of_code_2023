use std::fs;

#[derive(PartialEq, Clone)]
struct Cell {
    val: char,
    x: usize,
    y: usize,
    // nearby: [Option<&'a Cell<'a>>; 2],
}

impl Cell {
    fn empty() -> Cell {
        Cell {
            val: ' ',
            x: 0,
            y: 0,
            // nearby: [None, None],
        }
    }
}

struct Grid {
    grid: Vec<Vec<Cell>>,
    startx: usize,
    starty: usize,
    height: usize,
    width: usize,
}

impl Grid {
    fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if x >= self.height || y >= self.width {
            None
        } else {
            Some(&self.grid[x][y])
        }
    }

    fn get_nearby(&self, x: usize, y: usize) -> [Option<&Cell>; 2] {
        let val = self.get_cell(x, y).unwrap().val;
        match val {
            '|' => [self.get_cell(x - 1, y), self.get_cell(x + 1, y)],
            '-' => [self.get_cell(x, y - 1), self.get_cell(x, y + 1)],
            'L' => [self.get_cell(x - 1, y), self.get_cell(x, y + 1)],
            'J' => [self.get_cell(x - 1, y), self.get_cell(x, y - 1)],
            'F' => [self.get_cell(x + 1, y), self.get_cell(x, y + 1)],
            '7' => [self.get_cell(x + 1, y), self.get_cell(x, y - 1)],
            'S' => [self.get_cell(x - 1, y), self.get_cell(x + 1, y)],
            '.' => [None, None],
            c => panic!("unexpected char {}", c),
        }
    }

    fn get_nearby_cell(&self, cell: &Cell) -> [Option<&Cell>; 2] {
        self.get_nearby(cell.x, cell.y)
    }

    fn new(contents: &str) -> Grid {
        let height = contents.lines().count();
        let width = contents.lines().nth(0).unwrap().chars().count();
        let mut grid: Vec<Vec<Cell>> = vec![vec![Cell::empty(); width]; height];
        let mut startx = 0;
        let mut starty = 0;
        for (i, line) in contents.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                let val = c;
                match val {
                    'S' => {
                        startx = i;
                        starty = j;
                    },
                    _ => (),
                }
                grid[i][j] = Cell { val, x: i, y: j };
            }
        }

        Grid {
            grid,
            startx, starty,
            height,
            width,
        }
    }
}

struct GridIter<'a> {
    grid: &'a Grid,
    curr: Option<&'a Cell>,
    prev: Option<&'a Cell>,
}

impl<'a> Iterator for GridIter<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.prev.is_none() {
            // starting
            self.prev = self.curr;
            self.curr = self.grid.get_nearby_cell(self.curr.unwrap())[0];
            return self.curr;
        }
        let cur = self.curr.unwrap();
        let nearby = self.grid.get_nearby_cell(cur);

        let start = self.grid.get_cell(self.grid.startx, self.grid.starty).unwrap();

        if cur == start {
            // end
            return None;
        }

        if nearby[0] == self.prev {
            self.prev = self.curr;
            self.curr = nearby[1];
        } else {
            self.prev = self.curr;
            self.curr = nearby[0];
        }
        self.curr
    }
}

fn main() {
    let file_path = "./input";
    let contents = fs::read_to_string(file_path).expect("[Error] Could not read input");

    let grid = Grid::new(&contents);
    
    let grid_iter = GridIter {
        grid : &grid,
        curr : Some(grid.get_cell(grid.startx, grid.starty).unwrap()),
        prev : None,
    };
    let grid_count = grid_iter.count() ;

    println!("Part 1 ans => {}", grid_count/ 2);


    // Part 2
    let grid_iter = GridIter {
        grid : &grid,
        curr : Some(grid.get_cell(grid.startx, grid.starty).unwrap()),
        prev : None,
    };

    let mut grid_points : Vec<&Cell> = grid_iter.collect();
    grid_points.push(grid_points[0]);

    let area : i64 = grid_points.windows(2).map(|p| p[0].x as i64 * p[1].y as i64 - p[0].y as i64 * p[1].x as i64).sum::<i64>().abs() / 2;
    let boundary = grid_count as i64;

    println!("Part 2 ans => {}", area - boundary / 2  + 1);



}
