use std::collections::HashSet;

use cp_rs::io::*;
type Grid = Vec<Vec<Tile>>;
#[allow(unused_mut)]
fn main() {
    let mut io = Io::from_file("day24.txt");
    let mut grid: Grid = vec![];
    for mut line in io.line_io() {
        let mut grid_line = vec![];
        for c in line.chars() {
            grid_line.push(match c {
                '.' => Tile::Void,
                '#' => Tile::Wall,
                '^' => Tile::Blizzards(vec![Blizzard::North]),
                '<' => Tile::Blizzards(vec![Blizzard::West]),
                'v' => Tile::Blizzards(vec![Blizzard::South]),
                '>' => Tile::Blizzards(vec![Blizzard::East]),
                _ => unreachable!(),
            });
        }
        grid.push(grid_line);
    }
    let height = grid.len();
    let width = grid[0].len();
    let res = moves_for_path(&mut grid, (0, 1), (height - 1, width - 2));
    let back = moves_for_path(&mut grid, (height - 1, width - 2), (0, 1));
    let res2 = res + back + moves_for_path(&mut grid, (0, 1), (height - 1, width - 2));
    io.write("Part 1: ");
    io.writeln(res);
    io.write("Part 2: ");
    io.writeln(res2);
}

fn grid_move(grid: &Grid) -> Grid {
    let mut res = grid.clone();
    // remove blizzards from new grid
    for r in 0..res.len() {
        for c in 0..res[0].len() {
            match res[r][c] {
                Tile::Void => (),
                Tile::Wall => (),
                Tile::Blizzards(_) => res[r][c] = Tile::Void,
            }
    }
}
    // add blizzards to new grid
    for (r, row) in grid.iter().enumerate() {
        for (c, elem) in row.iter().enumerate() {
            match elem {
                Tile::Void => (),
                Tile::Wall => (),
                Tile::Blizzards(blizzards) => {
                    for b in blizzards {
                        match b {
                            Blizzard::North => {
                                let (nr, nc) = (if r > 1 { r - 1 } else {grid.len() - 2}, c);
                                if let Tile::Blizzards(x) = &mut res[nr][nc] {
                                    x.push(Blizzard::North);
                                } else {
                                    res[nr][nc] = Tile::Blizzards(vec![Blizzard::North]);
                                }
                            }
                            Blizzard::West => {
                                let (nr, nc) = (r, if c > 1 { c - 1 } else {grid[0].len() - 2});
                                if let Tile::Blizzards(x) = &mut res[nr][nc] {
                                    x.push(Blizzard::West);
                                } else {
                                    res[nr][nc] = Tile::Blizzards(vec![Blizzard::West]);
                                }
                            }
                            Blizzard::South => {
                                let (nr, nc) = (if r < grid.len() - 2 { r + 1 } else {1}, c);
                                if let Tile::Blizzards(x) = &mut res[nr][nc] {
                                    x.push(Blizzard::South);
                                } else {
                                    res[nr][nc] = Tile::Blizzards(vec![Blizzard::South]);
                                }
                            }
                            Blizzard::East => {
                                let (nr, nc) = (r, if c < grid[0].len() - 2 { c + 1 } else {1});
                                if let Tile::Blizzards(x) = &mut res[nr][nc] {
                                    x.push(Blizzard::East);
                                } else {
                                    res[nr][nc] = Tile::Blizzards(vec![Blizzard::East]);
                                }
                            }
                        }
                }
            }
            }
        }
    }
    res
}

fn moves_for_path(grid: &mut Grid, from: (usize, usize), to: (usize, usize)) -> usize {
    let mut new_pos: HashSet<(usize, usize)> = HashSet::new();
    new_pos.insert(from);
    let mut new_grid = grid_move(&grid);
    let mut depth = 0;
    loop {
        let values = new_pos.drain().collect::<Vec<(usize, usize)>>();
        for (r, c) in values {
            if r == to.0 && c == to.1 {
                return depth;
            }
            // stay
            match new_grid[r][c] {
                Tile::Void => {new_pos.insert((r, c));}
                _ => (),
            }
            // right
            match new_grid[r][c + 1] {
                Tile::Void => {new_pos.insert((r, c + 1));}
                _ => (),
            }
            // left
            match new_grid[r][c - 1] {
                Tile::Void => {new_pos.insert((r, c - 1));}
                _ => (),
            }
            // up
            if r > 0 {match new_grid[r - 1][c] {
                Tile::Void => {new_pos.insert((r - 1, c));}
                _ => (),
            }}
            // down
            if r < grid.len() - 1 {match new_grid[r + 1][c] {
                Tile::Void => {new_pos.insert((r + 1, c));}
                _ => (),
            }}
        }
        *grid = new_grid;
        new_grid = grid_move(&grid);
        depth += 1;
    }
}

#[derive(Debug, Clone)]
enum Tile {
    Void,
    Wall,
    Blizzards(Vec<Blizzard>),
}

#[derive(Debug, Copy, Clone)]
enum Blizzard {
    North,
    West,
    South,
    East,
}
