use cp_rs::io::*;
use std::collections::HashMap;
const TILES: [Tile; 5] = [
    Tile::HorizontalLine,
    Tile::Cross,
    Tile::L,
    Tile::VerticalLine,
    Tile::Square,
];

#[allow(unused_mut)]
fn main() {
    let mut io = Io::from_file("day17.txt");
    // let mut io = Io::from_file("test.txt");
    // let mut io = Io::from_str(">");
    let mut moves = vec![];
    let mut grid: Vec<[bool; 7]> = Vec::new();
    grid.push([true; 7]);
    for c in io.chars() {
        match c {
            '>' => moves.push(Move::Right),
            '<' => moves.push(Move::Left),
            _ => unreachable!(),
        }
    }
    let mut tidx = 0;
    let mut curr_tile = Tile::HorizontalLine;
    let mut row: usize;
    let mut col: usize;
    let mut store: HashMap<([[bool; 7]; 15], usize, Tile), (usize, usize, usize, usize)> =
        HashMap::new();
    let mut max_row: usize = 0;
    for _ in 0..15 {
        grid.push([false; 7]);
    }
    (row, col) = (max_row + 4, 2);
    let mut midx = 0;
    let mut inserted: usize = 0;
    let (mut m1, mut m2, mut i1, mut i2) = (0, 0, 0, 0);
    let mut repeated = false;
    while inserted < 1000000000000 {
        let m = moves[midx % moves.len()];
        match m {
            Move::Left => {
                if !curr_tile.collides_left(&grid, row, col) {
                    col -= 1;
                }
            }
            Move::Right => {
                if !curr_tile.collides_right(&grid, row, col) {
                    col += 1;
                }
            }
        }
        if curr_tile.collides_down(&grid, row, col) {
            (max_row, tidx, curr_tile) = insert(&mut grid, row, col, max_row, curr_tile, tidx);
            inserted += 1;
            if !repeated {
                let mut last10 = [[false; 7]; 15];
                for row in grid.len() - 15..grid.len() {
                    let lrow = row + 15 - grid.len();
                    for c in 0..7 {
                        last10[lrow][c] = grid[row][c];
                    }
                }
                if let Some((max1, max2, ins1, ins2)) =
                    store.get_mut(&(last10, midx % moves.len(), curr_tile))
                {
                    io.writeln(*max1);
                    io.writeln(*max2);
                    io.writeln(*ins1);
                    io.writeln(*ins2);
                    m1 = *max1;
                    m2 = max_row;
                    i1 = *ins1;
                    i2 = inserted;
                    inserted += (1000000000000 - i2) / (i2 - i1) * (i2 - i1);
                    repeated = true;
                } else {
                    store.insert(
                        (last10, midx % moves.len(), curr_tile),
                        (max_row, 0, inserted, 0),
                    );
                }
            }
            let height_diff: isize =
                max_row as isize + 4 + curr_tile.height() - grid.len() as isize;
            for _ in 0..height_diff {
                grid.push([false; 7]);
            }
            (row, col) = (max_row + 4, 2);
        } else {
            row -= 1;
        }
        midx += 1;
    }

    print_grid(&grid);
    let mut res: usize = (1000000000000 - i2) / (i2 - i1) * (m2 - m1);
    io.write("Part 1: ");
    io.writeln(res);
    io.write("Part 2: ");
    io.writeln(res + max_row);
}

fn print_grid(grid: &Vec<[bool; 7]>) {
    for row in grid.iter().rev() {
        for b in row {
            print!("{}", if *b { '#' } else { '.' });
        }
        print!("\n");
    }
}

/// returns (top heigth taken, new tidx, new tile)
fn insert(
    grid: &mut Vec<[bool; 7]>,
    row: usize,
    col: usize,
    mut max_row: usize,
    mut tile: Tile,
    mut tidx: usize,
) -> (usize, usize, Tile) {
    tile.insert_into_grid(grid, row, col);
    let height = tile.height() as usize;
    if height + row - 1 > max_row {
        max_row = height + row - 1;
    }
    tidx += 1;
    tile = TILES[tidx % 5];
    (max_row, tidx, tile)
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Tile {
    HorizontalLine,
    Cross,
    L,
    VerticalLine,
    /// position is at bottom left, no tile there
    Square,
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Left,
    Right,
}

impl Tile {
    /// Checks if the tile would collide if it would move downward from its current position (row,
    /// col)
    fn collides_down(&self, grid: &Vec<[bool; 7]>, row: usize, col: usize) -> bool {
        if row <= 1 {
            return true;
        }
        match self {
            Tile::HorizontalLine => {
                grid[row - 1][col]
                    || grid[row - 1][col + 1]
                    || grid[row - 1][col + 2]
                    || grid[row - 1][col + 3]
            }
            Tile::Cross => grid[row][col] || grid[row - 1][col + 1] || grid[row][col + 2],
            Tile::L => grid[row - 1][col] || grid[row - 1][col + 1] || grid[row - 1][col + 2],
            Tile::VerticalLine => grid[row - 1][col],
            Tile::Square => grid[row - 1][col] || grid[row - 1][col + 1],
        }
    }

    fn collides_right(&self, grid: &Vec<[bool; 7]>, row: usize, col: usize) -> bool {
        match self {
            Tile::HorizontalLine => col >= 3 || grid[row][col + 4],
            Tile::Cross => {
                col >= 4 || grid[row][col + 2] || grid[row + 1][col + 3] || grid[row + 2][col + 2]
            }
            Tile::L => {
                col >= 4 || grid[row][col + 3] || grid[row + 1][col + 3] || grid[row + 2][col + 3]
            }
            Tile::VerticalLine => {
                col >= 6
                    || grid[row][col + 1]
                    || grid[row + 1][col + 1]
                    || grid[row + 2][col + 1]
                    || grid[row + 3][col + 1]
            }
            Tile::Square => col >= 5 || grid[row][col + 2] || grid[row + 1][col + 2],
        }
    }
    fn collides_left(&self, grid: &Vec<[bool; 7]>, row: usize, col: usize) -> bool {
        if col == 0 {
            return true;
        }
        match self {
            Tile::HorizontalLine => grid[row][col - 1],
            Tile::Cross => grid[row][col] || grid[row + 1][col - 1] || grid[row + 2][col],
            Tile::L => grid[row][col - 1] || grid[row + 1][col + 1] || grid[row + 2][col + 1],
            Tile::VerticalLine => {
                grid[row][col - 1]
                    || grid[row + 1][col - 1]
                    || grid[row + 2][col - 1]
                    || grid[row + 3][col - 1]
            }
            Tile::Square => grid[row][col - 1] || grid[row + 1][col - 1],
        }
    }
    fn insert_into_grid(&self, grid: &mut Vec<[bool; 7]>, row: usize, col: usize) {
        match self {
            Tile::HorizontalLine => {
                grid[row][col] = true;
                grid[row][col + 1] = true;
                grid[row][col + 2] = true;
                grid[row][col + 3] = true;
            }
            Tile::Cross => {
                grid[row + 1][col] = true;
                grid[row][col + 1] = true;
                grid[row + 1][col + 1] = true;
                grid[row + 2][col + 1] = true;
                grid[row + 1][col + 2] = true;
            }
            Tile::L => {
                grid[row][col] = true;
                grid[row][col + 1] = true;
                grid[row][col + 2] = true;
                grid[row + 1][col + 2] = true;
                grid[row + 2][col + 2] = true;
            }
            Tile::VerticalLine => {
                grid[row][col] = true;
                grid[row + 1][col] = true;
                grid[row + 2][col] = true;
                grid[row + 3][col] = true;
            }
            Tile::Square => {
                grid[row][col] = true;
                grid[row + 1][col] = true;
                grid[row][col + 1] = true;
                grid[row + 1][col + 1] = true;
            }
        }
    }
    fn height(&self) -> isize {
        match self {
            Tile::HorizontalLine => 1,
            Tile::Cross => 3,
            Tile::L => 3,
            Tile::VerticalLine => 4,
            Tile::Square => 2,
        }
    }
}
