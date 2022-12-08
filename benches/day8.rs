use cp_rs::io::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
type Grid<T> = Vec<Vec<T>>;

fn bench(input: String) -> String {
    let mut output = Vec::new();
    {
        let mut io = Io::with_reader_and_writer(input.as_bytes(), &mut output);
        let mut res = 0;
        let mut res2 = 0;
        let mut grid: Grid<usize> = Vec::new();
        for mut line in io.line_io() {
            let chars = line.chars();
            let row: Vec<usize> = chars
                .into_iter()
                .map(|x| x as usize - '0' as usize)
                .collect();
            grid.push(row);
        }
        for r in 1..grid.len() - 1 {
            for c in 1..grid[0].len() - 1 {
                let val = grid[r][c];
                // top
                let mut top_visible = true;
                let mut distt: usize = r;
                for (i, r2) in (0..r).rev().enumerate() {
                    if r2 != r && grid[r2][c] >= val {
                        top_visible = false;
                        distt = i + 1;
                        break;
                    }
                }
                // down
                let mut down_visible = true;
                let mut distd: usize = grid.len() - r - 1;
                for (i, r2) in (r + 1..grid.len()).enumerate() {
                    if r2 != r && grid[r2][c] >= val {
                        down_visible = false;
                        distd = i + 1;
                        break;
                    }
                }
                // left
                let mut left_visible = true;
                let mut distl: usize = c;
                for (i, c2) in (0..c).rev().enumerate() {
                    if c2 != c && grid[r][c2] >= val {
                        left_visible = false;
                        distl = i + 1;
                        break;
                    }
                }
                // right
                let mut right_visible = true;
                let mut distr: usize = grid[0].len() - c - 1;
                for (i, c2) in (c + 1..grid[0].len()).enumerate() {
                    if c2 != c && grid[r][c2] >= val {
                        right_visible = false;
                        distr = i + 1;
                        break;
                    }
                }
                if top_visible || down_visible || left_visible || right_visible {
                    res += 1;
                }
                let score = distr * distl * distt * distd;
                if score > res2 {
                    res2 = score;
                }
            }
        }
        res += 2 * grid.len() + 2 * grid[0].len() - 4;

        io.write("Part 1: ");
        io.writeln(res);
        io.write("Part 2: ");
        io.writeln(res2);
    }
    std::str::from_utf8(output.as_slice()).unwrap().to_string()
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("day8", |b| {
        b.iter(|| bench(black_box(include_str!("../day8.txt").to_string())))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
