use cp_rs::io::*;
use std::collections::HashSet;
type Grid = [[[bool; 22]; 22]; 22];
type ReachableGrid = [[[Reachable; 22]; 22]; 22];
fn main() {
    let input = std::fs::File::open("day18.txt").unwrap();
    let mut stdout = std::io::stdout();
    let mut io = Io::with_reader_and_writer(input, &mut stdout);
    let all = io.read_all();
    let input: &[u8] = all.as_bytes();
    let mut grid = [[[false; 22]; 22]; 22];
    let mut reachable = [[[Reachable::Unknown; 22]; 22]; 22];
    let mut cubes = vec![];
    let mut idx = 0;
    while idx < input.len() {
        let x = get_digit(&input, &mut idx) + 1;
        let y = get_digit(&input, &mut idx) + 1;
        let z = get_digit_end(&input, &mut idx) + 1;
        grid[x][y][z] = true;
        cubes.push((x, y, z));
    }
    let mut res = 0;
    for (x, y, z) in cubes {
        if !grid[x - 1][y][z] {
            res += 1;
        }
        if !grid[x + 1][y][z] {
            res += 1;
        }
        if !grid[x][y - 1][z] {
            res += 1;
        }
        if !grid[x][y + 1][z] {
            res += 1;
        }
        if !grid[x][y][z - 1] {
            res += 1;
        }
        if !grid[x][y][z + 1] {
            res += 1;
        }
    }
    let res1 = res;
    for x in 1..21 {
        for y in 1..21 {
            for z in 1..21 {
                if !grid[x][y][z] && reachable[x][y][z] == Reachable::NotReachable {
                    if grid[x - 1][y][z] {
                        res -= 1;
                    }
                    if grid[x + 1][y][z] {
                        res -= 1;
                    }
                    if grid[x][y - 1][z] {
                        res -= 1;
                    }
                    if grid[x][y + 1][z] {
                        res -= 1;
                    }
                    if grid[x][y][z - 1] {
                        res -= 1;
                    }
                    if grid[x][y][z + 1] {
                        res -= 1;
                    }
                    continue;
                }
                if grid[x][y][z] || reachable[x][y][z] != Reachable::Unknown {
                    continue;
                }
                if reach_edge(x, y, z, &grid, &mut reachable) {
                    if grid[x - 1][y][z] {
                        res -= 1;
                    }
                    if grid[x + 1][y][z] {
                        res -= 1;
                    }
                    if grid[x][y - 1][z] {
                        res -= 1;
                    }
                    if grid[x][y + 1][z] {
                        res -= 1;
                    }
                    if grid[x][y][z - 1] {
                        res -= 1;
                    }
                    if grid[x][y][z + 1] {
                        res -= 1;
                    }
                }
            }
        }
    }
    io.write("Part 1: ");
    io.writeln(res1);
    io.write("Part 2: ");
    io.writeln(res);
}

fn reach_edge(x: usize, y: usize, z: usize, grid: &Grid, reachable: &mut ReachableGrid) -> bool {
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push((x, y, z));
    while !queue.is_empty() {
        let (x, y, z) = queue.pop().unwrap();
        visited.insert((x, y, z));
        if is_edge(x, y, z) {
            for (x, y, z) in visited {
                reachable[x][y][z] = Reachable::Reachable;
            }
            return false;
        }
        match reachable[x][y][z] {
            Reachable::Unknown => (),
            Reachable::Reachable => {
                for (x, y, z) in visited {
                    reachable[x][y][z] = Reachable::Reachable;
                }
                return true;
            }
            Reachable::NotReachable => {
                for (x, y, z) in visited {
                    reachable[x][y][z] = Reachable::NotReachable;
                }
                return true;
            }
        }
        if !visited.contains(&(x - 1, y, z)) && !grid[x - 1][y][z] {
            queue.push((x - 1, y, z));
        }
        if !visited.contains(&(x + 1, y, z)) && !grid[x + 1][y][z] {
            queue.push((x + 1, y, z));
        }
        if !visited.contains(&(x, y - 1, z)) && !grid[x][y - 1][z] {
            queue.push((x, y - 1, z));
        }
        if !visited.contains(&(x, y + 1, z)) && !grid[x][y + 1][z] {
            queue.push((x, y + 1, z));
        }
        if !visited.contains(&(x, y, z - 1)) && !grid[x][y][z - 1] {
            queue.push((x, y, z - 1));
        }
        if !visited.contains(&(x, y, z + 1)) && !grid[x][y][z + 1] {
            queue.push((x, y, z + 1));
        }
    }
    for (x, y, z) in visited {
        reachable[x][y][z] = Reachable::NotReachable;
    }
    return true;
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Reachable {
    Unknown,
    Reachable,
    NotReachable,
}

fn is_edge(x: usize, y: usize, z: usize) -> bool {
    x <= 1 || x >= 21 || y <= 1 || y >= 21 || z <= 1 || z >= 21
}

fn get_digit(input: &[u8], idx: &mut usize) -> usize {
    let mut val = 0;
    while input[*idx] != b',' {
        val *= 10;
        val += (input[*idx] - b'0') as usize;
        *idx += 1;
    }
    *idx += 1;
    val
}
fn get_digit_end(input: &[u8], idx: &mut usize) -> usize {
    let mut val = 0;
    while input[*idx] != b'\n' {
        val *= 10;
        val += (input[*idx] - b'0') as usize;
        *idx += 1;
    }
    *idx += 1;
    val
}
