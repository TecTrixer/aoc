use cp_rs::io::*;
fn main() {
    let mut io = Io::from_file("day14.txt");
    let mut res = 0;
    let mut res2 = 0;
    let mut grid = [[false; 1000]; 200];
    let mut max_row = 0;
    for line in io.lines() {
        let (mut old_col, mut old_row) = (usize::MAX, usize::MAX);
        for coords in line.split(" -> ") {
            let (col, row): (usize, usize) = Io::from_str(coords).tuple();
            if old_col == usize::MAX {
                old_col = col;
                old_row = row;
                grid[row][col] = true;
                if row > max_row {
                    max_row = row;
                }
            } else if old_col == col {
                if old_row > row {
                    for i in row..old_row {
                        grid[i][col] = true;
                    }
                } else {
                    for i in old_row + 1..=row {
                        grid[i][col] = true;
                    }
                }
                old_row = row;
                if row > max_row {
                    max_row = row;
                }
            } else if old_row == row {
                if old_col > col {
                    for i in col..old_col {
                        grid[row][i] = true;
                    }
                } else {
                    for i in old_col + 1..=col {
                        grid[row][i] = true;
                    }
                }
                old_col = col;
            } else {
                unreachable!();
            }
        }
    }
    for i in 0..1000 {
        grid[max_row + 2][i] = true;
    }
    let mut solved1 = false;
    loop {
        let (mut col, mut row) = (500, 0);
        grid[0][500] = true;
        while row <= max_row + 2 {
            if !grid[row + 1][col] {
                grid[row][col] = false;
                grid[row + 1][col] = true;
                row += 1;
            } else if !grid[row + 1][col - 1] {
                grid[row][col] = false;
                grid[row + 1][col - 1] = true;
                row += 1;
                col -= 1;
            } else if !grid[row + 1][col + 1] {
                grid[row][col] = false;
                grid[row + 1][col + 1] = true;
                row += 1;
                col += 1;
            } else {
                if !solved1 {
                    res += 1;
                }
                res2 += 1;
                break;
            }
        }
        if row > max_row && !solved1 {
            solved1 = true;
            res -= 1;
        }
        if row > max_row + 2 || grid[0][500] {
            break;
        }
    }
    io.write("Part 1: ");
    io.writeln(res);
    io.write("Part 2: ");
    io.writeln(res2);
}
