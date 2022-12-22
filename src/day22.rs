use cp_rs::io::*;

const CUBE_SIZE: usize = 50;
type Grid = [[Tile; 152]; 202];
fn main() {
    let mut io = Io::from_file("day22.txt");
    let mut grid: Grid = [[Tile::Void; 152]; 202];
    let mut blocks = io.blocks();
    for (row, line) in blocks[0].lines().into_iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                ' ' => (),
                '.' => grid[row + 1][col + 1] = Tile::Open,
                '#' => grid[row + 1][col + 1] = Tile::Wall,
                c => {
                    dbg!(row, col, c);
                    unreachable!();
                }
            }
        }
    }
    let mut moves = vec![];
    let mut val = 0;
    for c in blocks[1].chars() {
        match c {
            'R' => {
                moves.push(Move::Go(val));
                moves.push(Move::Right);
                val = 0;
            }
            'L' => {
                moves.push(Move::Go(val));
                moves.push(Move::Left);
                val = 0;
            }
            c if c.is_digit(10) => {
                val *= 10;
                val += c as usize - '0' as usize;
            }
            c => {
                dbg!(c);
                unreachable!();
            }
        }
    }
    moves.push(Move::Go(val));
    let (mut row, mut col) = find_start(&grid);
    let mut facing = Facing::Right;
    for m in moves.iter() {
        (row, col, facing) = exec_move(*m, &grid, row, col, facing);
    }
    let facing_val = match facing {
        Facing::Up => 3,
        Facing::Right => 0,
        Facing::Left => 2,
        Facing::Down => 1,
    };
    let res = 1000 * row + 4 * col + facing_val;
    let (mut row, mut col) = find_start(&grid);
    let mut facing = Facing::Right;
    for m in moves {
        (row, col, facing) = exec_move2(m, &grid, row, col, facing);
    }
    let facing_val = match facing {
        Facing::Up => 3,
        Facing::Right => 0,
        Facing::Left => 2,
        Facing::Down => 1,
    };
    let res2 = 1000 * row + 4 * col + facing_val;
    io.write("Part 1: ");
    io.writeln(res);
    io.write("Part 2: ");
    io.writeln(res2);
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Tile {
    Void,
    Wall,
    Open,
}

fn find_start(grid: &Grid) -> (usize, usize) {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == Tile::Open {
                return (row, col);
            }
        }
    }
    unreachable!();
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Move {
    Go(usize),
    Right,
    Left,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Facing {
    Up,
    Right,
    Left,
    Down,
}

fn exec_move(
    m: Move,
    grid: &Grid,
    mut row: usize,
    mut col: usize,
    facing: Facing,
) -> (usize, usize, Facing) {
    match m {
        Move::Go(x) => {
            if facing == Facing::Right {
                'outer: for _ in 0..x {
                    match grid[row][col + 1] {
                        Tile::Void => {
                            for pot_col in 0..col {
                                if grid[row][pot_col] == Tile::Void {
                                    continue;
                                } else if grid[row][pot_col] == Tile::Open {
                                    col = pot_col;
                                    break;
                                } else {
                                    break 'outer;
                                }
                            }
                        }
                        Tile::Wall => break,
                        Tile::Open => col += 1,
                    }
                }
            } else if facing == Facing::Left {
                'outer: for _ in 0..x {
                    match grid[row][col - 1] {
                        Tile::Void => {
                            for pot_col in (col..grid[0].len()).rev() {
                                if grid[row][pot_col] == Tile::Void {
                                    continue;
                                } else if grid[row][pot_col] == Tile::Open {
                                    col = pot_col;
                                    break;
                                } else {
                                    break 'outer;
                                }
                            }
                        }
                        Tile::Wall => break,
                        Tile::Open => col -= 1,
                    }
                }
            } else if facing == Facing::Down {
                'outer: for _ in 0..x {
                    match grid[row + 1][col] {
                        Tile::Void => {
                            for pot_row in 0..row {
                                if grid[pot_row][col] == Tile::Void {
                                    continue;
                                } else if grid[pot_row][col] == Tile::Open {
                                    row = pot_row;
                                    break;
                                } else {
                                    break 'outer;
                                }
                            }
                        }
                        Tile::Wall => break,
                        Tile::Open => row += 1,
                    }
                }
            } else if facing == Facing::Up {
                'outer: for _ in 0..x {
                    match grid[row - 1][col] {
                        Tile::Void => {
                            for pot_row in (row..grid.len()).rev() {
                                if grid[pot_row][col] == Tile::Void {
                                    continue;
                                } else if grid[pot_row][col] == Tile::Open {
                                    row = pot_row;
                                    break;
                                } else {
                                    break 'outer;
                                }
                            }
                        }
                        Tile::Wall => break,
                        Tile::Open => row -= 1,
                    }
                }
            }
            (row, col, facing)
        }
        Move::Right => {
            let new_facing = match facing {
                Facing::Up => Facing::Right,
                Facing::Right => Facing::Down,
                Facing::Left => Facing::Up,
                Facing::Down => Facing::Left,
            };
            (row, col, new_facing)
        }
        Move::Left => {
            let new_facing = match facing {
                Facing::Up => Facing::Left,
                Facing::Right => Facing::Up,
                Facing::Left => Facing::Down,
                Facing::Down => Facing::Right,
            };
            (row, col, new_facing)
        }
    }
}

fn exec_move2(
    m: Move,
    grid: &Grid,
    mut row: usize,
    mut col: usize,
    mut facing: Facing,
) -> (usize, usize, Facing) {
    match m {
        Move::Go(x) => {
            for _ in 0..x {
                if facing == Facing::Right {
                    match grid[row][col + 1] {
                        Tile::Void => {
                            // 2 -> 4
                            if row <= CUBE_SIZE {
                                match grid[3 * CUBE_SIZE + 1 - row][2 * CUBE_SIZE] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        col = 2 * CUBE_SIZE;
                                        row = 3 * CUBE_SIZE + 1 - row;
                                        facing = Facing::Left;
                                    }
                                }
                            // 3 -> 2
                            } else if row <= 2 * CUBE_SIZE {
                                match grid[CUBE_SIZE][CUBE_SIZE + row] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        col = CUBE_SIZE + row;
                                        row = CUBE_SIZE;
                                        facing = Facing::Up;
                                    }
                                }
                            // 4 -> 2
                            } else if row <= 3 * CUBE_SIZE {
                                match grid[3 * CUBE_SIZE + 1 - row][3 * CUBE_SIZE] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        col = 3 * CUBE_SIZE;
                                        row = 3 * CUBE_SIZE + 1 - row;
                                        facing = Facing::Left;
                                    }
                                }
                            // 6 -> 4
                            } else {
                                match grid[3 * CUBE_SIZE][row - 2 * CUBE_SIZE] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        col = row - 2 * CUBE_SIZE;
                                        row = 3 * CUBE_SIZE;
                                        facing = Facing::Up;
                                    }
                                }
                            }
                        }
                        Tile::Wall => break,
                        Tile::Open => col += 1,
                    }
                } else if facing == Facing::Left {
                    match grid[row][col - 1] {
                        Tile::Void => {
                            // 1 -> 5
                            if row <= CUBE_SIZE {
                                match grid[3 * CUBE_SIZE + 1 - row][1] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = 3 * CUBE_SIZE + 1 - row;
                                        col = 1;
                                        facing = Facing::Right;
                                    }
                                }
                            // 3 -> 5
                            } else if row <= 2 * CUBE_SIZE {
                                match grid[2 * CUBE_SIZE + 1][row - CUBE_SIZE] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        col = row - CUBE_SIZE;
                                        row = 2 * CUBE_SIZE + 1;
                                        facing = Facing::Down;
                                    }
                                }
                            // 5 -> 1
                            } else if row <= 3 * CUBE_SIZE {
                                match grid[3 * CUBE_SIZE + 1 - row][CUBE_SIZE + 1] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = 3 * CUBE_SIZE + 1 - row;
                                        col = CUBE_SIZE + 1;
                                        facing = Facing::Right;
                                    }
                                }
                            // 6 -> 1
                            } else {
                                match grid[1][row - 2 * CUBE_SIZE] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        col = row - 2 * CUBE_SIZE;
                                        row = 1;
                                        facing = Facing::Down;
                                    }
                                }
                            }
                        }
                        Tile::Wall => break,
                        Tile::Open => col -= 1,
                    }
                } else if facing == Facing::Down {
                    match grid[row + 1][col] {
                        Tile::Void => {
                            // 6 -> 2
                            if col <= CUBE_SIZE {
                                match grid[1][col + 2 * CUBE_SIZE] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = 1;
                                        col = col + 2 * CUBE_SIZE;
                                        facing = Facing::Down;
                                    }
                                }
                            // 4 -> 6
                            } else if col <= 2 * CUBE_SIZE {
                                match grid[col + 2 * CUBE_SIZE][CUBE_SIZE] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = col + 2 * CUBE_SIZE;
                                        col = CUBE_SIZE;
                                        facing = Facing::Left;
                                    }
                                }
                            // 2 -> 3
                            } else if col <= 3 * CUBE_SIZE {
                                match grid[col - CUBE_SIZE][2 * CUBE_SIZE] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = col - CUBE_SIZE;
                                        col = 2 * CUBE_SIZE;
                                        facing = Facing::Left;
                                    }
                                }
                            }
                        }
                        Tile::Wall => break,
                        Tile::Open => row += 1,
                    }
                } else if facing == Facing::Up {
                    match grid[row - 1][col] {
                        Tile::Void => {
                            // 5 -> 3
                            if col <= CUBE_SIZE {
                                match grid[col + CUBE_SIZE][CUBE_SIZE + 1] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = col + CUBE_SIZE;
                                        col = CUBE_SIZE + 1;
                                        facing = Facing::Right;
                                    }
                                }
                            // 1 -> 6
                            } else if col <= 2 * CUBE_SIZE {
                                match grid[col + 2 * CUBE_SIZE][1] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = col + 2 * CUBE_SIZE;
                                        col = 1;
                                        facing = Facing::Right;
                                    }
                                }
                            // 2 -> 6
                            } else {
                                match grid[4 * CUBE_SIZE][col - 2 * CUBE_SIZE] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = 4 * CUBE_SIZE;
                                        col = col - 2 * CUBE_SIZE;
                                        facing = Facing::Up;
                                    }
                                }
                            }
                        }
                        Tile::Wall => break,
                        Tile::Open => row -= 1,
                    }
                }
            }
            (row, col, facing)
        }
        Move::Right => {
            let new_facing = match facing {
                Facing::Up => Facing::Right,
                Facing::Right => Facing::Down,
                Facing::Left => Facing::Up,
                Facing::Down => Facing::Left,
            };
            (row, col, new_facing)
        }
        Move::Left => {
            let new_facing = match facing {
                Facing::Up => Facing::Left,
                Facing::Right => Facing::Up,
                Facing::Left => Facing::Down,
                Facing::Down => Facing::Right,
            };
            (row, col, new_facing)
        }
    }
}

fn _exec_move2(
    m: Move,
    grid: &Grid,
    mut row: usize,
    mut col: usize,
    mut facing: Facing,
) -> (usize, usize, Facing) {
    match m {
        Move::Go(x) => {
            for _ in 0..x {
                if facing == Facing::Right {
                    match grid[row][col + 1] {
                        Tile::Void => {
                            // 1 -> 6
                            if row <= CUBE_SIZE {
                                match grid[grid.len() - row][grid[0].len() - 1] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = grid.len() - row;
                                        col = grid[0].len() - 1;
                                        facing = Facing::Left;
                                    }
                                }
                            // 4 -> 6
                            } else if row <= 2 * CUBE_SIZE {
                                match grid[2 * CUBE_SIZE + 1][5 * CUBE_SIZE + 1 - row] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        col = 5 * CUBE_SIZE + 1 - row;
                                        row = 2 * CUBE_SIZE + 1;
                                        facing = Facing::Down;
                                    }
                                }
                            // 6 -> 1
                            } else {
                                match grid[3 * CUBE_SIZE + 1 - row][3 * CUBE_SIZE] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        col = 3 * CUBE_SIZE;
                                        row = 3 * CUBE_SIZE + 1 - row;
                                        facing = Facing::Left;
                                    }
                                }
                            }
                        }
                        Tile::Wall => break,
                        Tile::Open => col += 1,
                    }
                } else if facing == Facing::Left {
                    match grid[row][col - 1] {
                        Tile::Void => {
                            // 1 -> 3
                            if row <= CUBE_SIZE {
                                match grid[CUBE_SIZE + 1][CUBE_SIZE + row] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = CUBE_SIZE + 1;
                                        col = CUBE_SIZE + row;
                                        facing = Facing::Down;
                                    }
                                }
                            // 2 -> 6
                            } else if row <= 2 * CUBE_SIZE {
                                match grid[3 * CUBE_SIZE][5 * CUBE_SIZE + 1 - row] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        col = 5 * CUBE_SIZE + 1 - row;
                                        row = 3 * CUBE_SIZE;
                                        facing = Facing::Up;
                                    }
                                }
                            // 5 -> 3
                            } else {
                                match grid[2 * CUBE_SIZE][4 * CUBE_SIZE + 1 - row] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = 2 * CUBE_SIZE;
                                        col = 4 * CUBE_SIZE + 1 - row;
                                        facing = Facing::Up;
                                    }
                                }
                            }
                        }
                        Tile::Wall => break,
                        Tile::Open => col -= 1,
                    }
                } else if facing == Facing::Down {
                    match grid[row + 1][col] {
                        Tile::Void => {
                            // 2 -> 5
                            if col <= CUBE_SIZE {
                                match grid[3 * CUBE_SIZE][3 * CUBE_SIZE + 1 - col] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = 3 * CUBE_SIZE;
                                        col = 3 * CUBE_SIZE + 1 - col;
                                        facing = Facing::Up;
                                    }
                                }
                            // 3 -> 5
                            } else if col <= 2 * CUBE_SIZE {
                                match grid[4 * CUBE_SIZE + 1 - col][2 * CUBE_SIZE + 1] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = 4 * CUBE_SIZE + 1 - col;
                                        col = 2 * CUBE_SIZE + 1;
                                        facing = Facing::Right;
                                    }
                                }
                            // 5 -> 2
                            } else if col <= 3 * CUBE_SIZE {
                                match grid[2 * CUBE_SIZE][3 * CUBE_SIZE + 1 - col] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = 2 * CUBE_SIZE;
                                        col = 3 * CUBE_SIZE + 1 - col;
                                        facing = Facing::Up;
                                    }
                                }
                            // 6 -> 2
                            } else {
                                match grid[5 * CUBE_SIZE + 1 - col][1] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = 5 * CUBE_SIZE + 1 - col;
                                        col = 1;
                                        facing = Facing::Right;
                                    }
                                }
                            }
                        }
                        Tile::Wall => break,
                        Tile::Open => row += 1,
                    }
                } else if facing == Facing::Up {
                    match grid[row - 1][col] {
                        Tile::Void => {
                            // 2 -> 1
                            if col <= CUBE_SIZE {
                                match grid[1][3 * CUBE_SIZE + 1 - col] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = 1;
                                        col = 3 * CUBE_SIZE + 1 - col;
                                        facing = Facing::Down;
                                    }
                                }
                            // 3 -> 1
                            } else if col <= 2 * CUBE_SIZE {
                                match grid[col - CUBE_SIZE][2 * CUBE_SIZE + 1] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = col - CUBE_SIZE;
                                        col = 2 * CUBE_SIZE + 1;
                                        facing = Facing::Right;
                                    }
                                }
                            // 1 -> 2
                            } else if col <= 3 * CUBE_SIZE {
                                match grid[CUBE_SIZE + 1][3 * CUBE_SIZE + 1 - col] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = CUBE_SIZE + 1;
                                        col = 3 * CUBE_SIZE + 1 - col;
                                        facing = Facing::Down;
                                    }
                                }
                            // 6 -> 4
                            } else {
                                match grid[5 * CUBE_SIZE + 1 - col][3 * CUBE_SIZE] {
                                    Tile::Void => unreachable!(),
                                    Tile::Wall => break,
                                    Tile::Open => {
                                        row = 5 * CUBE_SIZE + 1 - col;
                                        col = 3 * CUBE_SIZE;
                                        facing = Facing::Left;
                                    }
                                }
                            }
                        }
                        Tile::Wall => break,
                        Tile::Open => row -= 1,
                    }
                }
            }
            (row, col, facing)
        }
        Move::Right => {
            let new_facing = match facing {
                Facing::Up => Facing::Right,
                Facing::Right => Facing::Down,
                Facing::Left => Facing::Up,
                Facing::Down => Facing::Left,
            };
            (row, col, new_facing)
        }
        Move::Left => {
            let new_facing = match facing {
                Facing::Up => Facing::Left,
                Facing::Right => Facing::Up,
                Facing::Left => Facing::Down,
                Facing::Down => Facing::Right,
            };
            (row, col, new_facing)
        }
    }
}
