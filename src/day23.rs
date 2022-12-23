use cp_rs::io::*;
use std::collections::{HashMap, HashSet};
const NORTH: [(isize, isize); 3] = [(-1, -1), (-1, 0), (-1, 1)];
const WEST: [(isize, isize); 3] = [(-1, -1), (0, -1), (1, -1)];
const SOUTH: [(isize, isize); 3] = [(1, -1), (1, 0), (1, 1)];
const EAST: [(isize, isize); 3] = [(-1, 1), (0, 1), (1, 1)];
const ALL: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
const DIR_LIST: [[(isize, isize); 3]; 4] = [NORTH, SOUTH, WEST, EAST];
fn main() {
    let mut io = Io::from_file("day23.txt");
    // let mut io = Io::from_file("test.txt");
    let mut grid: HashSet<(isize, isize)> = HashSet::new();
    let mut dests: HashMap<(isize, isize), (usize, isize, isize)> = HashMap::new();
    for (row, mut line) in io.line_io().enumerate() {
        for (col, c) in line.chars().iter().enumerate() {
            match c {
                '#' => {
                    grid.insert((row as isize, col as isize));
                }
                '.' => (),
                _ => unreachable!(),
            }
        }
    }
    let mut wants_to_move = true;
    let mut idx = 0;
    let mut count = 0;
    while wants_to_move {
        wants_to_move = false;
        // First half, every elf proposes a new position
        for (r, c) in grid.iter() {
            let has_elv = ALL.iter().any(|&(rd, cd)| grid.contains(&(r + rd, c + cd)));
            if has_elv {
                for i in 0..4 {
                    let dir = DIR_LIST[(idx + i) % 4];
                    if dir
                        .iter()
                        .all(|&(rd, cd)| !grid.contains(&(r + rd, c + cd)))
                    {
                        let (nr, nc) = (r + dir[1].0, c + dir[1].1);
                        if let Some((x, or, oc)) = dests.get_mut(&(nr, nc)) {
                            *x += 1;
                            *or = *r;
                            *oc = *c;
                        } else {
                            dests.insert((nr, nc), (1, *r, *c));
                        }
                        wants_to_move = true;
                        break;
                    }
                }
            }
        }
        // Second half, here we check if only one elv wants to move and then move
        for ((tr, tc), (x, fr, fc)) in dests.iter() {
            if *x == 1 {
                grid.remove(&(*fr, *fc));
                grid.insert((*tr, *tc));
            }
        }
        dests.clear();
        idx = (idx + 1) % 4;
        count += 1;
        if count == 10 {
            io.write("Part 1: ");
            io.writeln(get_empty_fields(&grid));
        }
    }
    io.write("Part 2: ");
    io.writeln(count);
}

fn _print(grid: &HashSet<(isize, isize)>, count: usize) {
    let mut minx = isize::MAX;
    let mut maxx = isize::MIN;
    let mut miny = isize::MAX;
    let mut maxy = isize::MIN;

    println!("Round {}:", count);
    for &(y, x) in grid.iter() {
        if x < minx {
            minx = x;
        }
        if x > maxx {
            maxx = x;
        }
        if y < miny {
            miny = y;
        }
        if y > maxy {
            maxy = y;
        }
    }
    for row in miny..=maxy {
        for col in minx..=maxx {
            let c = if let Some(_) = grid.get(&(row, col)) {
                '#'
            } else {
                ','
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}
fn get_empty_fields(grid: &HashSet<(isize, isize)>) -> isize {
    let mut minx = isize::MAX;
    let mut maxx = isize::MIN;
    let mut miny = isize::MAX;
    let mut maxy = isize::MIN;

    for &(y, x) in grid.iter() {
        if x < minx {
            minx = x;
        }
        if x > maxx {
            maxx = x;
        }
        if y < miny {
            miny = y;
        }
        if y > maxy {
            maxy = y;
        }
    }
    let x = maxx - minx + 1;
    let y = maxy - miny + 1;
    x * y - grid.len() as isize
}
